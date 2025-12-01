use anyhow::{Ok, Result, anyhow};
use rand::{rngs::StdRng, Rng, SeedableRng};
use rand_distr::{Distribution, Normal};
use rayon::prelude::*;
use statrs::statistics::{Data, Distribution as StatDist, Median, OrderStatistics};
use slint::SharedString;

use super::portfolio::{Portfolio, PortfolioStats, TickerStats};
use super::barriers::{calculate_hit_statistics, check_barriers};
use std::collections::HashMap;



// Temporary SimParams struct (will be replaced by Slint-generated type later)
#[derive(Debug, Clone)]
pub struct SimParams {
    pub initial_price: f32,
    pub horizon: i32,
    pub num_paths: i32,
    pub seed: i32,
    pub use_antithetic: bool,
    pub dt: f32,
    pub model_type: SharedString,
    
    // GBM and Jump Diffusion parameters
    pub mu: f32,
    pub sigma: f32,
    
    // Jump Diffusion specific
    pub lambda: f32,
    pub mu_j: f32,
    pub sigma_j: f32,
    
    // GARCH parameters
    pub omega: f32,
    pub alpha: f32,
    pub beta: f32,
}

// Model-specific parameters enum
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ModelParams {
    GBM {
        mu: f64,
        sigma: f64,
    },
    Bootstrap {

    },
    JumpDiffusion {
        mu: f64,         
        sigma: f64,       
        lambda: f64,      
        mu_j: f64,        
        sigma_j: f64,     
    },
    GARCH {
        omega: f64,      
        alpha: f64,       
        beta: f64,       
    },
}

#[derive(Debug, Clone)]
pub struct SimStats {
    pub model: String,
    pub paths: usize,
    pub horizon: usize,
    pub mean: f64,
    pub std_dev: f64,
    pub median: f64,
    pub p5: f64,
    pub p25: f64,
    pub p75: f64,
    pub p95: f64,
    pub var95: f64,
    pub sharpe_ratio: f64,
    pub best_case: f64,
    pub worst_case: f64,
    pub max_drawdown: f64,
}

pub fn run_simulation (params: SimParams, hist_log_returns: Vec<f64>,) -> Result<(SimStats, (Vec<u8>, u32, u32), (Vec<u8>, u32, u32))> {
    let init_price = params.initial_price as f64;
    let mu = params.mu as f64;
    let sigma = params.sigma as f64;
    let horizon = params.horizon as usize;
    let num_paths = params.num_paths as usize;
    let dt = params.dt as f64;
    let model_name = match params.model_type.as_str() {
        "GBM" => "GBM",
        "Bootstrap" => "Bootstrap",
        _ => "",
    };

    let paths: Vec<Vec<f64>> = (0..num_paths).into_par_iter().map(|i| {
        let seed = (params.seed as u64).wrapping_add(i as u64);
        let mut rng = StdRng::seed_from_u64(seed);

        match params.model_type.as_str() {
            "GBM" => generate_gbm_path(init_price, mu, sigma, horizon, dt, params.use_antithetic && (i%2==1), &mut rng),
            "Bootstrap" => generate_bootstrap_path(init_price, horizon, &hist_log_returns, &mut rng),
            "JumpDiffusion" => {
                let mu = params.mu as f64;
                let sigma = params.sigma as f64;
                let lambda = params.lambda as f64;
                let mu_j = params.mu_j as f64;
                let sigma_j = params.sigma_j as f64;
                generate_jump_diffusion_path(init_price, mu, sigma, lambda, mu_j, sigma_j, horizon, dt, params.use_antithetic && (i%2==1), &mut rng)
            }
            "GARCH" => {
                let omega = params.omega as f64;
                let alpha = params.alpha as f64;
                let beta = params.beta as f64;
                generate_garch_path(init_price, omega, alpha, beta, horizon, dt, params.use_antithetic && (i%2==1), &mut rng)
            }
    _ => Vec::new()
}
    }).collect();

    let mut terminal_prices: Vec<f64> = paths.iter().map(|path| *path.last().unwrap()).collect();
    let stats = calculate_statistics(&mut terminal_prices, &paths, model_name, num_paths, horizon, init_price)?;

    let paths_png = super::plotting::plot_price_paths(
        &paths,
        &params.model_type,
    )?;
    let hist_png = super::plotting::plot_histogram(&terminal_prices, 100)?;

    Ok((stats, paths_png, hist_png))
}

fn generate_gbm_path(init_price: f64, mu: f64, sigma: f64, steps: usize, dt: f64, is_antithetic: bool, rng: &mut StdRng,) -> Vec<f64> {
    //plus 1 for init_price
    let mut path = Vec::with_capacity(steps+1);
    path.push(init_price);
    let mut current_price = init_price;

    let drift = (mu - 0.5 * sigma.powi(2)) * dt;
    let diffusion = sigma * dt.sqrt();
    let normal = Normal::new(0.0, 1.0).unwrap();

    for _ in 0..steps {
        let mut z = normal.sample(rng);
        if is_antithetic {
            z = -z;
        }

        let next_price = current_price * (drift + diffusion * z).exp();
        path.push(next_price);
        current_price = next_price;
    }
    path
}

fn generate_bootstrap_path(init_price: f64, steps: usize, log_returns: &[f64], rng: &mut StdRng) -> Vec<f64> {
    if log_returns.is_empty() {
        return vec![init_price; steps+1];
    }

    let mut path = Vec::with_capacity(steps+1);
    path.push(init_price);
    let mut current_price = init_price;

    for _ in 0..steps {
        let idx = rng.random_range(0..log_returns.len());
        let log_return = log_returns[idx];
        let next_price = current_price * log_return.exp();
        path.push(next_price);
        current_price = next_price;
    }
    path
}

pub fn estimate_paramaters(log_returns: &[f64]) -> Result<(f64, f64)> {
    if log_returns.len() < 2 {
        return Err(anyhow!("Not enough data to estimate parameters. Neet at least 2 log returns."));
    }
    let data = Data::new(log_returns.to_vec());
    let mu = data.mean().unwrap_or(0.0);
    let sigma = data.std_dev().unwrap_or(0.0);

    Ok((mu, sigma))
}

fn calculate_statistics(terminal_prices: &mut [f64], paths: &[Vec<f64>], model: &str, num_paths: usize, horizon: usize, init_price: f64) -> Result<SimStats> {
    if terminal_prices.is_empty() {
        return Err(anyhow!("No terminal prcies to analyze"));
    }

    let data = Data::new(terminal_prices.to_vec());
    let mean = data.mean().unwrap_or(0.0);
    let std_dev = data.std_dev().unwrap_or(0.0);
    let median = data.median();

    let mut ordered_data = Data::new(terminal_prices.to_vec());
    let p5 = ordered_data.percentile(5);
    let p25 = ordered_data.percentile(25);
    let p75 = ordered_data.percentile(75);
    let p95 = ordered_data.percentile(95);

    // Best Case and Worst Case (highest and lowest terminal prices)
    let best_case = terminal_prices.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let worst_case = terminal_prices.iter().fold(f64::INFINITY, |a, &b| a.min(b));

    let returns: Vec<f64> = terminal_prices.iter()
        .map(|&price| (price - init_price) / init_price)
        .collect();
    
    let mut returns_data = Data::new(returns.clone());
    let p5_return = returns_data.percentile(5);
    let var95 = -p5_return;

    // Calculate Sharpe Ratio: (Mean Return - Risk Free Rate) / Std Dev Return
    // Using Risk Free Rate = 0 for simplicity (can be made configurable later)
    let mean_return = returns_data.mean().unwrap_or(0.0);
    let std_return = returns_data.std_dev().unwrap_or(0.0);
    let sharpe_ratio = if std_return > 0.0 {
        mean_return / std_return
    } else {
        0.0
    };

    // Calculate Max Drawdown: maximum peak-to-trough decline across all paths
    let mut max_drawdown = 0.0;
    for path in paths {
        let mut peak = init_price;
        let mut max_dd = 0.0;
        
        for &price in path {
            if price > peak {
                peak = price;
            }
            let drawdown = (peak - price) / peak;
            if drawdown > max_dd {
                max_dd = drawdown;
            }
        }
        
        if max_dd > max_drawdown {
            max_drawdown = max_dd;
        }
    }

    Ok(SimStats { 
        model: model.to_string(), 
        paths: num_paths, 
        horizon, 
        mean, 
        std_dev, 
        median, 
        p5, 
        p25, 
        p75, 
        p95, 
        var95, 
        sharpe_ratio,
        best_case,
        worst_case,
        max_drawdown,
    })

}

// Helper function to create ModelParams from Slint's SimParams
pub fn create_model_params(model_type: &str, mu: f64, sigma: f64) -> ModelParams {
    match model_type {
        "GBM" => ModelParams::GBM { mu, sigma },
        "Bootstrap" => ModelParams::Bootstrap {},
        "JumpDiffusion" => ModelParams::JumpDiffusion {
            mu,
            sigma,
            lambda: 2.0,      // Default: 2 jumps per year
            mu_j: -0.02,      // Default: small negative jump
            sigma_j: 0.05,    // Default: 5% jump volatility
        },
        "GARCH" => ModelParams::GARCH {
            omega: 0.00001,   // Default: small constant
            alpha: 0.1,       // Default: ARCH coefficient
            beta: 0.85,       // Default: GARCH coefficient
        },
        _ => ModelParams::GBM { mu, sigma }, // Default fallback
    }
}

fn generate_jump_diffusion_path(
    init_price: f64,
    mu: f64,           // Drift
    sigma: f64,        // Diffusion volatility
    lambda: f64,       // Jump intensity (average jumps per unit time)
    mu_j: f64,         // Mean of jump size (in log space)
    sigma_j: f64,      // Std dev of jump size (in log space)
    steps: usize,
    dt: f64,
    is_antithetic: bool,
    rng: &mut StdRng,
) -> Vec<f64> {
    let mut path = Vec::with_capacity(steps + 1);
    path.push(init_price);
    let mut current_price = init_price;

    // GBM components
    let drift = (mu - 0.5 * sigma.powi(2)) * dt;
    let diffusion = sigma * dt.sqrt();
    let normal = Normal::new(0.0, 1.0).unwrap();

    // Jump components
    use rand_distr::Poisson;
    let poisson = Poisson::new(lambda * dt).unwrap();
    let jump_normal = Normal::new(mu_j, sigma_j).unwrap();

    for _ in 0..steps {
        // Diffusion part (GBM)
        let mut z = normal.sample(rng);
        if is_antithetic {
            z = -z;
        }
        
        let gbm_return = drift + diffusion * z;

        // Jump part
        let num_jumps = poisson.sample(rng) as usize;
        let mut jump_effect = 0.0;
        
        for _ in 0..num_jumps {
            // Jump size in log space
            let jump_size = jump_normal.sample(rng);
            jump_effect += jump_size;
        }

        // Combine: S_{t+1} = S_t * exp(gbm_return + jump_effect)
        let total_return = gbm_return + jump_effect;
        let next_price = current_price * total_return.exp();
        
        path.push(next_price);
        current_price = next_price;
    }
    
    path
}


fn generate_garch_path(
    init_price: f64,
    omega: f64,        // Constant term
    alpha: f64,        // ARCH coefficient
    beta: f64,         // GARCH coefficient
    steps: usize,
    dt: f64,
    is_antithetic: bool,
    rng: &mut StdRng,
) -> Vec<f64> {
    let mut path = Vec::with_capacity(steps + 1);
    path.push(init_price);
    let mut current_price = init_price;

    // Initialize variance (unconditional variance if stationary)
    let mut variance = if alpha + beta < 1.0 {
        omega / (1.0 - alpha - beta)
    } else {
        omega / 0.1  // Fallback if not stationary
    };
    
    let mut prev_return: f64 = 0.0;
    let normal = Normal::new(0.0, 1.0).unwrap();

    for _ in 0..steps {
        // Generate random shock
        let mut epsilon = normal.sample(rng);
        if is_antithetic {
            epsilon = -epsilon;
        }

        // Current return: r_t = σ_t * ε_t
        let volatility = variance.sqrt();
        let return_t = volatility * epsilon * dt.sqrt();

        // Update price: S_t = S_{t-1} * exp(r_t)
        let next_price = current_price * return_t.exp();
        
        path.push(next_price);

        // Update variance for next step: σ²_{t+1} = ω + α·r²_t + β·σ²_t
        variance = omega + alpha * prev_return.powi(2) + beta * variance;
        
        // Prevent variance from becoming too small or negative
        variance = variance.max(1e-6);
        
        prev_return = return_t;
        current_price = next_price;
    }
    
    path
}

/// Run portfolio simulation with multiple tickers
pub fn run_portfolio_simulation(
    portfolio: &Portfolio,
    horizon: usize,
    num_paths: usize,
    seed: u64,
    use_antithetic: bool,
    dt: f64,
    hist_returns_map: HashMap<String, Vec<f64>>,
) -> Result<(PortfolioStats, HashMap<String, Vec<Vec<f64>>>)> {
    
    if portfolio.tickers.is_empty() {
        return Err(anyhow!("Portfolio cannot be empty"));
    }

    // Validate portfolio
    portfolio.validate()?;

    // Generate paths for each ticker in parallel
    let ticker_paths: HashMap<String, Vec<Vec<f64>>> = portfolio.tickers
        .par_iter()
        .map(|ticker_config| {
            let ticker_symbol = ticker_config.symbol.clone();
            
            // Get historical returns for this ticker (if using Bootstrap)
            let hist_returns = hist_returns_map.get(&ticker_symbol).cloned().unwrap_or_default();
            
            // Generate paths for this ticker
            let paths: Vec<Vec<f64>> = (0..num_paths)
                .into_par_iter()
                .map(|i| {
                    let ticker_seed = seed.wrapping_add(i as u64).wrapping_add(ticker_symbol.len() as u64);
                    let mut rng = StdRng::seed_from_u64(ticker_seed);
                    
                    generate_path_for_ticker(
                        ticker_config,
                        horizon,
                        dt,
                        use_antithetic && (i % 2 == 1),
                        &hist_returns,
                        &mut rng,
                    )
                })
                .collect();
            
            (ticker_symbol, paths)
        })
        .collect();

    // Calculate portfolio statistics
    let portfolio_stats = calculate_portfolio_statistics(portfolio, &ticker_paths)?;

    Ok((portfolio_stats, ticker_paths))
}

/// Generate a single path for a ticker based on its model configuration
fn generate_path_for_ticker(
    ticker_config: &super::portfolio::TickerConfig,
    horizon: usize,
    dt: f64,
    is_antithetic: bool,
    hist_returns: &[f64],
    rng: &mut StdRng,
) -> Vec<f64> {
    let init_price = ticker_config.initial_price;
    
    match &ticker_config.model_params {
        ModelParams::GBM { mu, sigma } => {
            generate_gbm_path(init_price, *mu, *sigma, horizon, dt, is_antithetic, rng)
        }
        ModelParams::Bootstrap {} => {
            generate_bootstrap_path(init_price, horizon, hist_returns, rng)
        }
        ModelParams::JumpDiffusion { mu, sigma, lambda, mu_j, sigma_j } => {
            generate_jump_diffusion_path(init_price, *mu, *sigma, *lambda, *mu_j, *sigma_j, horizon, dt, is_antithetic, rng)
        }
        ModelParams::GARCH { omega, alpha, beta } => {
            generate_garch_path(init_price, *omega, *alpha, *beta, horizon, dt, is_antithetic, rng)
        }
    }
}

/// Calculate comprehensive portfolio statistics
fn calculate_portfolio_statistics(
    portfolio: &Portfolio,
    ticker_paths: &HashMap<String, Vec<Vec<f64>>>,
) -> Result<PortfolioStats> {
    
    if ticker_paths.is_empty() {
        return Err(anyhow!("No ticker paths to analyze"));
    }

    let num_paths = ticker_paths.values().next().unwrap().len();
    
    // Calculate per-ticker statistics
    let mut ticker_stats = HashMap::new();
    
    for ticker_config in &portfolio.tickers {
        let symbol = &ticker_config.symbol;
        let paths = ticker_paths.get(symbol).unwrap();
        
        let stats = calculate_ticker_statistics(ticker_config, paths)?;
        ticker_stats.insert(symbol.clone(), stats);
    }

    // Calculate portfolio-level statistics
    let portfolio_values = calculate_portfolio_values(portfolio, ticker_paths)?;
    let portfolio_returns = calculate_portfolio_returns(&portfolio_values, portfolio.total_capital);
    
    let portfolio_stats = calculate_portfolio_level_stats(&portfolio_returns, ticker_stats)?;
    
    Ok(portfolio_stats)
}

/// Calculate statistics for individual ticker
fn calculate_ticker_statistics(
    ticker_config: &super::portfolio::TickerConfig,
    paths: &[Vec<f64>],
) -> Result<TickerStats> {
    
    if paths.is_empty() {
        return Err(anyhow!("No paths for ticker {}", ticker_config.symbol));
    }

    // Extract final prices
    let final_prices: Vec<f64> = paths.iter()
        .map(|path| *path.last().unwrap())
        .collect();

    // Basic statistics
    let data = Data::new(final_prices.clone());
    let mean_final_price = data.mean().unwrap_or(0.0);
    let median_final_price = data.median();
    let std_dev = data.std_dev().unwrap_or(0.0);

    // Find best and worst final prices
    let best_final_price = final_prices.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let worst_final_price = final_prices.iter().fold(f64::INFINITY, |a, &b| a.min(b));

    // Calculate barrier hit statistics
    let (prob_hit_stoploss, prob_hit_target, avg_time_to_stoploss, avg_time_to_target) = 
        calculate_hit_statistics(paths, ticker_config.stop_loss, ticker_config.target);

    Ok(TickerStats {
        symbol: ticker_config.symbol.clone(),
        mean_final_price,
        median_final_price,
        std_dev,
        prob_hit_stoploss,
        prob_hit_target,
        avg_time_to_stoploss,
        avg_time_to_target,
        best_final_price,
        worst_final_price,
    })
}

/// Calculate portfolio values for each path
fn calculate_portfolio_values(
    portfolio: &Portfolio,
    ticker_paths: &HashMap<String, Vec<Vec<f64>>>,
) -> Result<Vec<Vec<f64>>> {
    
    let num_paths = ticker_paths.values().next().unwrap().len();
    let horizon = ticker_paths.values().next().unwrap()[0].len();
    
    let mut portfolio_values = vec![vec![0.0; horizon]; num_paths];
    
    for path_idx in 0..num_paths {
        for time_step in 0..horizon {
            let mut portfolio_value = 0.0;
            
            for ticker_config in &portfolio.tickers {
                let ticker_paths = ticker_paths.get(&ticker_config.symbol).unwrap();
                let ticker_price = ticker_paths[path_idx][time_step];
                
                // Calculate number of shares for this ticker
                let ticker_capital = portfolio.total_capital * ticker_config.weight;
                let shares = ticker_capital / ticker_config.initial_price;
                
                // Add to portfolio value
                portfolio_value += shares * ticker_price;
            }
            
            portfolio_values[path_idx][time_step] = portfolio_value;
        }
    }
    
    Ok(portfolio_values)
}

/// Calculate portfolio returns from portfolio values
fn calculate_portfolio_returns(
    portfolio_values: &[Vec<f64>],
    initial_capital: f64,
) -> Vec<f64> {
    portfolio_values.iter()
        .map(|path| {
            let final_value = *path.last().unwrap();
            (final_value - initial_capital) / initial_capital
        })
        .collect()
}

/// Calculate portfolio-level statistics
fn calculate_portfolio_level_stats(
    portfolio_returns: &[f64],
    ticker_stats: HashMap<String, TickerStats>,
) -> Result<PortfolioStats> {
    
    if portfolio_returns.is_empty() {
        return Err(anyhow!("No portfolio returns to analyze"));
    }

    // Basic portfolio statistics
    let data = Data::new(portfolio_returns.to_vec());
    let mean_portfolio_return = data.mean().unwrap_or(0.0);
    let median_portfolio_return = data.median();
    let std_portfolio_return = data.std_dev().unwrap_or(0.0);

    // Profit/Loss analysis
    let profitable_returns: Vec<f64> = portfolio_returns.iter()
        .filter(|&&r| r > 0.0)
        .copied()
        .collect();
    
    let loss_returns: Vec<f64> = portfolio_returns.iter()
        .filter(|&&r| r < 0.0)
        .copied()
        .collect();

    let prob_profit = profitable_returns.len() as f64 / portfolio_returns.len() as f64;
    let prob_loss = loss_returns.len() as f64 / portfolio_returns.len() as f64;

    let mean_profit = if !profitable_returns.is_empty() {
        profitable_returns.iter().sum::<f64>() / profitable_returns.len() as f64
    } else {
        0.0
    };

    let mean_loss = if !loss_returns.is_empty() {
        loss_returns.iter().sum::<f64>() / loss_returns.len() as f64
    } else {
        0.0
    };

    // Risk metrics
    let mut sorted_returns = portfolio_returns.to_vec();
    sorted_returns.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let var95_idx = (portfolio_returns.len() as f64 * 0.05) as usize;
    let var95 = -sorted_returns[var95_idx.min(sorted_returns.len() - 1)];

    // Maximum drawdown (simplified - using final return as proxy)
    let max_drawdown = sorted_returns[0].abs();

    Ok(PortfolioStats {
        mean_portfolio_return,
        median_portfolio_return,
        std_portfolio_return,
        prob_profit,
        prob_loss,
        mean_profit,
        mean_loss,
        var95,
        max_drawdown,
        ticker_stats,
    })
}
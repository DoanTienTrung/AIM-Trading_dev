use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::core_sim::ModelParams;

/// Configuration for a single ticker in portfolio
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TickerConfig {
    pub symbol: String,
    pub initial_price: f64,
    pub weight: f64,           // Portfolio weight (0.0 - 1.0)
    pub stop_loss: Option<f64>, // Stop loss price (optional)
    pub target: Option<f64>,    // Target price (optional)
    pub model_params: ModelParams, // Model configuration for this ticker
}

/// Portfolio containing multiple tickers
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Portfolio {
    pub tickers: Vec<TickerConfig>,
    pub total_capital: f64,
}

/// Statistics for individual ticker in portfolio
#[derive(Debug, Clone)]
pub struct TickerStats {
    pub symbol: String,
    pub mean_final_price: f64,
    pub median_final_price: f64,
    pub std_dev: f64,

    // Stop loss / Target tracking
    pub prob_hit_stoploss: f64,   // % paths that hit stop loss
    pub prob_hit_target: f64,      // % paths that hit target
    pub avg_time_to_stoploss: Option<f64>,  // Average days to hit SL
    pub avg_time_to_target: Option<f64>,    // Average days to hit target

    // Path performance
    pub best_final_price: f64,
    pub worst_final_price: f64,
}

/// Portfolio-level statistics
#[derive(Debug, Clone)]
pub struct PortfolioStats {
    // Portfolio-level metrics
    pub mean_portfolio_return: f64,
    pub median_portfolio_return: f64,
    pub std_portfolio_return: f64,

    // Probability metrics
    pub prob_profit: f64,          // % paths with positive return
    pub prob_loss: f64,            // % paths with negative return
    pub mean_profit: f64,          // Average profit when profitable
    pub mean_loss: f64,            // Average loss when loss occurs

    // Risk metrics
    pub var95: f64,                // Value at Risk 95%
    pub max_drawdown: f64,         // Maximum portfolio drawdown

    // Per-ticker statistics
    pub ticker_stats: HashMap<String, TickerStats>,
}

impl Portfolio {
    /// Create new empty portfolio
    pub fn new(total_capital: f64) -> Self {
        Self {
            tickers: Vec::new(),
            total_capital,
        }
    }

    /// Add ticker to portfolio
    pub fn add_ticker(&mut self, ticker_config: TickerConfig) -> Result<()> {
        // Check if ticker already exists
        if self.tickers.iter().any(|t| t.symbol == ticker_config.symbol) {
            return Err(anyhow::anyhow!("Ticker {} already exists in portfolio", ticker_config.symbol));
        }

        self.tickers.push(ticker_config);
        Ok(())
    }

    /// Remove ticker from portfolio
    pub fn remove_ticker(&mut self, symbol: &str) -> Result<()> {
        let initial_len = self.tickers.len();
        self.tickers.retain(|t| t.symbol != symbol);
        
        if self.tickers.len() == initial_len {
            return Err(anyhow::anyhow!("Ticker {} not found in portfolio", symbol));
        }
        
        Ok(())
    }

    /// Get total weight of all tickers
    pub fn total_weight(&self) -> f64 {
        self.tickers.iter().map(|t| t.weight).sum()
    }

    /// Validate portfolio configuration
    pub fn validate(&self) -> Result<()> {
        if self.tickers.is_empty() {
            return Err(anyhow::anyhow!("Portfolio must contain at least one ticker"));
        }

        let total_weight = self.total_weight();
        if (total_weight - 1.0).abs() > 0.001 {
            return Err(anyhow::anyhow!("Portfolio weights must sum to 1.0, got {:.3}", total_weight));
        }

        for ticker in &self.tickers {
            if ticker.weight <= 0.0 || ticker.weight > 1.0 {
                return Err(anyhow::anyhow!("Ticker {} weight must be between 0 and 1, got {}", ticker.symbol, ticker.weight));
            }

            if ticker.initial_price <= 0.0 {
                return Err(anyhow::anyhow!("Ticker {} initial price must be positive, got {}", ticker.symbol, ticker.initial_price));
            }

            // Validate stop loss and target
            if let Some(stop_loss) = ticker.stop_loss {
                if stop_loss >= ticker.initial_price {
                    return Err(anyhow::anyhow!("Ticker {} stop loss ({}) must be less than initial price ({})", 
                        ticker.symbol, stop_loss, ticker.initial_price));
                }
            }

            if let Some(target) = ticker.target {
                if target <= ticker.initial_price {
                    return Err(anyhow::anyhow!("Ticker {} target ({}) must be greater than initial price ({})", 
                        ticker.symbol, target, ticker.initial_price));
                }
            }
        }

        Ok(())
    }

    /// Auto-balance weights equally
    pub fn auto_balance_weights(&mut self) {
        if !self.tickers.is_empty() {
            let equal_weight = 1.0 / self.tickers.len() as f64;
            for ticker in &mut self.tickers {
                ticker.weight = equal_weight;
            }
        }
    }

    /// Get ticker by symbol
    pub fn get_ticker(&self, symbol: &str) -> Option<&TickerConfig> {
        self.tickers.iter().find(|t| t.symbol == symbol)
    }

    /// Get mutable ticker by symbol
    pub fn get_ticker_mut(&mut self, symbol: &str) -> Option<&mut TickerConfig> {
        self.tickers.iter_mut().find(|t| t.symbol == symbol)
    }
}

impl Default for Portfolio {
    fn default() -> Self {
        Self::new(100000.0) // Default $100k capital
    }
}
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use super::core_sim::ModelParams;
use super::portfolio::Portfolio;

/// Configuration for simulation (supports both single ticker and portfolio)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SimConfig {
    // Configuration version for backward compatibility
    #[serde(default = "default_version")]
    pub version: u32,
    
    // Simulation parameters
    pub horizon: usize,
    pub num_paths: usize,
    pub seed: u64,
    pub use_antithetic: bool,
    pub dt: f64,
    
    // Portfolio configuration (new approach)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio: Option<Portfolio>,
    
    // Legacy single ticker support (for backward compatibility)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gbm_params: Option<GBMParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jump_diffusion_params: Option<JumpDiffusionParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub garch_params: Option<GARCHParams>,
}

fn default_version() -> u32 {
    2 // Version 2 = portfolio support
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GBMParams {
    pub mu: f64,
    pub sigma: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JumpDiffusionParams {
    pub mu: f64,
    pub sigma: f64,
    pub lambda: f64,
    pub mu_j: f64,
    pub sigma_j: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GARCHParams {
    pub omega: f64,
    pub alpha: f64,
    pub beta: f64,
}

impl SimConfig {
    /// Create new portfolio-based config
    pub fn new_portfolio(
        horizon: usize,
        num_paths: usize,
        seed: u64,
        use_antithetic: bool,
        portfolio: Portfolio,
    ) -> Self {
        Self {
            version: 2,
            horizon,
            num_paths,
            seed,
            use_antithetic,
            dt: 1.0,
            portfolio: Some(portfolio),
            // Legacy fields set to None
            initial_price: None,
            model_type: None,
            gbm_params: None,
            jump_diffusion_params: None,
            garch_params: None,
        }
    }

    /// Create legacy single ticker config (for backward compatibility)
    pub fn new_single_ticker(
        initial_price: f64,
        horizon: usize,
        num_paths: usize,
        seed: u64,
        use_antithetic: bool,
        model_type: String,
        model_params: ModelParams,
    ) -> Result<Self> {
        let mut config = Self {
            version: 1,
            horizon,
            num_paths,
            seed,
            use_antithetic,
            dt: 1.0,
            portfolio: None,
            initial_price: Some(initial_price),
            model_type: Some(model_type.clone()),
            gbm_params: None,
            jump_diffusion_params: None,
            garch_params: None,
        };

        // Set model-specific parameters
        match model_params {
            ModelParams::GBM { mu, sigma } => {
                config.gbm_params = Some(GBMParams { mu, sigma });
            }
            ModelParams::Bootstrap {} => {
                // No additional params needed
            }
            ModelParams::JumpDiffusion { mu, sigma, lambda, mu_j, sigma_j } => {
                config.jump_diffusion_params = Some(JumpDiffusionParams { mu, sigma, lambda, mu_j, sigma_j });
            }
            ModelParams::GARCH { omega, alpha, beta } => {
                config.garch_params = Some(GARCHParams { omega, alpha, beta });
            }
        }

        Ok(config)
    }

    /// Check if this is a portfolio configuration
    pub fn is_portfolio(&self) -> bool {
        self.portfolio.is_some()
    }

    /// Convert legacy config to ModelParams enum (for backward compatibility)
    pub fn to_model_params(&self) -> Result<ModelParams> {
        let model_type = self.model_type.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Model type not specified"))?;

        match model_type.as_str() {
            "GBM" => {
                if let Some(ref params) = self.gbm_params {
                    Ok(ModelParams::GBM {
                        mu: params.mu,
                        sigma: params.sigma,
                    })
                } else {
                    Err(anyhow::anyhow!("GBM parameters not found"))
                }
            }
            "Bootstrap" => Ok(ModelParams::Bootstrap {}),
            "JumpDiffusion" => {
                if let Some(ref params) = self.jump_diffusion_params {
                    Ok(ModelParams::JumpDiffusion {
                        mu: params.mu,
                        sigma: params.sigma,
                        lambda: params.lambda,
                        mu_j: params.mu_j,
                        sigma_j: params.sigma_j,
                    })
                } else {
                    Err(anyhow::anyhow!("Jump Diffusion parameters not found"))
                }
            }
            "GARCH" => {
                if let Some(ref params) = self.garch_params {
                    Ok(ModelParams::GARCH {
                        omega: params.omega,
                        alpha: params.alpha,
                        beta: params.beta,
                    })
                } else {
                    Err(anyhow::anyhow!("GARCH parameters not found"))
                }
            }
            _ => Err(anyhow::anyhow!("Unknown model type: {}", model_type)),
        }
    }
}

/// Save configuration to JSON file
pub fn save_config(config: &SimConfig, path: &Path) -> Result<()> {
    let json = serde_json::to_string_pretty(config)?;
    fs::write(path, json)?;
    Ok(())
}

/// Load configuration from JSON file
pub fn load_config(path: &Path) -> Result<SimConfig> {
    let json = fs::read_to_string(path)?;
    let config: SimConfig = serde_json::from_str(&json)?;
    Ok(config)
}

/// Validate configuration
pub fn validate_config(config: &SimConfig) -> Result<()> {
    // Basic validations
    if config.horizon == 0 {
        return Err(anyhow::anyhow!("Horizon must be greater than 0"));
    }
    
    if config.num_paths == 0 {
        return Err(anyhow::anyhow!("Number of paths must be greater than 0"));
    }
    
    if config.dt <= 0.0 {
        return Err(anyhow::anyhow!("dt must be positive"));
    }
    
    // Validate based on configuration type
    if let Some(ref portfolio) = config.portfolio {
        // Portfolio validation
        portfolio.validate()?;
    } else {
        // Legacy single ticker validation
        if let Some(initial_price) = config.initial_price {
            if initial_price <= 0.0 {
                return Err(anyhow::anyhow!("Initial price must be positive"));
            }
        }
        
        if let Some(ref model_type) = config.model_type {
            validate_legacy_model_params(config, model_type)?;
        }
    }
    
    Ok(())
}

/// Validate legacy model parameters
fn validate_legacy_model_params(config: &SimConfig, model_type: &str) -> Result<()> {
    match model_type {
        "GBM" => {
            if let Some(ref params) = config.gbm_params {
                if params.sigma < 0.0 {
                    return Err(anyhow::anyhow!("GBM sigma must be non-negative"));
                }
            } else {
                return Err(anyhow::anyhow!("GBM parameters missing"));
            }
        }
        "JumpDiffusion" => {
            if let Some(ref params) = config.jump_diffusion_params {
                if params.lambda < 0.0 {
                    return Err(anyhow::anyhow!("Jump Diffusion lambda must be non-negative"));
                }
                if params.sigma < 0.0 {
                    return Err(anyhow::anyhow!("Jump Diffusion sigma must be non-negative"));
                }
                if params.sigma_j < 0.0 {
                    return Err(anyhow::anyhow!("Jump Diffusion sigma_j must be non-negative"));
                }
            } else {
                return Err(anyhow::anyhow!("Jump Diffusion parameters missing"));
            }
        }
        "GARCH" => {
            if let Some(ref params) = config.garch_params {
                if params.omega <= 0.0 {
                    return Err(anyhow::anyhow!("GARCH omega must be positive"));
                }
                if params.alpha < 0.0 {
                    return Err(anyhow::anyhow!("GARCH alpha must be non-negative"));
                }
                if params.beta < 0.0 {
                    return Err(anyhow::anyhow!("GARCH beta must be non-negative"));
                }
                if params.alpha + params.beta >= 1.0 {
                    return Err(anyhow::anyhow!("GARCH stationarity condition failed: alpha + beta must be < 1"));
                }
            } else {
                return Err(anyhow::anyhow!("GARCH parameters missing"));
            }
        }
        "Bootstrap" => {
            // No additional validation needed
        }
        _ => {
            return Err(anyhow::anyhow!("Unknown model type: {}", model_type));
        }
    }
    
    Ok(())
}
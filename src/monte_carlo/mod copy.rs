// Module exports for Monte Carlo Simulation
pub mod core_sim;
pub mod data_io;
pub mod portfolio;
pub mod barriers;
pub mod plotting;
pub mod config;

// Re-export commonly used items
pub use core_sim::{ModelParams, SimStats, estimate_paramaters, run_portfolio_simulation, run_simulation, create_model_params};
pub use data_io::{get_ticker_info, load_all_records, StockRecord};
pub use portfolio::{Portfolio, PortfolioStats, TickerConfig};
pub use config::{SimConfig, GBMParams, JumpDiffusionParams, GARCHParams, save_config, load_config, validate_config};
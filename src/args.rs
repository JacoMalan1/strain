use std::str::FromStr;

use clap::Parser;

/// A CPU stressing utility written in Rust
#[derive(Debug, Clone, Parser)]
#[command(version, about)]
pub struct StrainArgs {
    /// Name of the stressing strategy to use.
    /// See --list-strategies for list of strategies.
    #[arg(short, long, default_value_t = String::from("lucas-lehmer"))]
    pub strategy: String,

    /// List the available stressing strategies.
    #[arg(long)]
    pub list_strategies: bool,

    /// Enable debug logging.
    #[arg(short, long, default_value_t = false)]
    pub debug: bool,

    /// Number of stressing threads to run. (Default is CPU logical core count)
    #[arg(short, long, default_value = None)]
    pub threads: Option<usize>,

    /// Mandelbrot test step size
    #[arg(short = 'S', long, default_value_t = f64::EPSILON)]
    pub mandelbrot_step_size: f64,

    /// Mandelbrot test threshold value
    #[arg(short = 'T', long, default_value_t = 1e6)]
    pub mandelbrot_threshold: f64,

    /// Mandelbrot test precision (in bits)
    #[arg(short = 'P', long, default_value_t = 128)]
    pub mandelbrot_precision: u32,
}

#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum StressingStrategies {
    LucasLehmer,
    Mandelbrot,
}

impl From<StressingStrategies> for &'static str {
    fn from(value: StressingStrategies) -> Self {
        match value {
            StressingStrategies::LucasLehmer => "lucas-lehmer",
            StressingStrategies::Mandelbrot => "mandelbrot",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParseStrategyError {
    pub message: String,
}

impl From<&str> for ParseStrategyError {
    fn from(value: &str) -> Self {
        Self {
            message: String::from(value),
        }
    }
}

impl FromStr for StressingStrategies {
    type Err = ParseStrategyError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "lucas-lehmer" => Ok(Self::LucasLehmer),
            "mandelbrot" => Ok(Self::Mandelbrot),
            _ => Err(ParseStrategyError::from("No such stressing strategy")),
        }
    }
}

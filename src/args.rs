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
}

#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum StressingStrategies {
    LucasLehmer,
}

impl From<StressingStrategies> for &'static str {
    fn from(value: StressingStrategies) -> Self {
        match value {
            StressingStrategies::LucasLehmer => "lucas-lehmer",
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
            _ => Err(ParseStrategyError::from("No such stressing strategy")),
        }
    }
}

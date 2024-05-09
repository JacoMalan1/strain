use crate::{
    args::{StrainArgs, StressingStrategies},
    lucas_lehmer::LucasLehmer,
    mandelbrot::Mandelbrot,
    rsa::RSA,
    stress::StressStrategy,
};
use clap::Parser;
use log::LevelFilter;
use std::{io::Write, str::FromStr, time::SystemTime};

pub mod args;
pub mod lucas_lehmer;
pub mod mandelbrot;
pub mod rsa;
pub mod stress;

fn setup_logger(log_level: log::LevelFilter) -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log_level)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

fn main() {
    let args = StrainArgs::parse();
    setup_logger(if args.debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    })
    .expect("Failed to setup logger");

    if args.list_strategies {
        print!(include_str!("strategies.txt"));
        std::io::stdout()
            .flush()
            .expect("Failed to write to stdout");
        return;
    }

    let requested_strategy =
        StressingStrategies::from_str(&args.strategy).expect("Invalid stressing strategy");

    let threads = args.threads.unwrap_or(
        std::thread::available_parallelism()
            .expect("Failed to get number of available threads")
            .into(),
    );

    let mut strategy: Box<dyn StressStrategy> = match requested_strategy {
        StressingStrategies::LucasLehmer => Box::new(LucasLehmer::new(threads)),
        StressingStrategies::Mandelbrot => Box::new(Mandelbrot::new(
            threads,
            args.mandelbrot_precision,
            rug::Float::with_val(args.mandelbrot_precision, args.mandelbrot_step_size),
            rug::Float::with_val(args.mandelbrot_precision, args.mandelbrot_threshold),
            args.mandelbrot_iterations,
        )),
        StressingStrategies::RSA => Box::new(RSA::new(threads)),
    };

    log::info!("Starting {}...", strategy.name());
    strategy.run();
}

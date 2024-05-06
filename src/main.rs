use crate::stress::{LucasLehmer, StressStrategy};

mod stress;

fn main() {
    let threads =
        std::thread::available_parallelism().expect("Failed to get number of available threads");
    let strategy = LucasLehmer::new(threads.into());

    println!("Starting {}...", strategy.name());
    strategy.run();
}

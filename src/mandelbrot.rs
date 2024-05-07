use rug::ops::{CompleteRound, Pow};

use crate::stress::StressStrategy;

#[derive(Debug)]
pub struct Mandelbrot {
    num_threads: usize,
    precision: u32,
    step_size: rug::Float,
    threshold: rug::Float,
}

impl Mandelbrot {
    pub fn new(
        num_threads: usize,
        precision: u32,
        step_size: rug::Float,
        threshold: rug::Float,
    ) -> Self {
        Self {
            num_threads,
            precision,
            step_size,
            threshold,
        }
    }

    pub fn worker(
        real_range: (rug::Float, rug::Float),
        imag_range: (rug::Float, rug::Float),
        precision: u32,
        step_size: rug::Float,
        threshold: rug::Float,
    ) {
        let mut current = rug::Complex::with_val(precision, (real_range.0, imag_range.0));

        while current.real() < &real_range.1 {
            while current.imag() < &imag_range.1 {
                let mut z = rug::Complex::with_val(precision, (0.0, 0.0));

                while z.abs_ref().complete((precision, precision)).real() < &threshold {
                    z = z.pow(2) + &current;
                }

                current += rug::Complex::with_val(
                    precision,
                    (rug::Float::with_val(precision, 0.0), &step_size),
                );
            }
            current += rug::Complex::with_val(
                precision,
                (&step_size, rug::Float::with_val(precision, 0.0)),
            );
        }
    }
}

impl StressStrategy for Mandelbrot {
    fn run(&mut self) {
        let mut workers = vec![];
        for i in 0..self.num_threads {
            let slice = self.num_threads as f64 / i as f64;
            let real_range = (
                rug::Float::with_val(self.precision, -2.0 + slice * i as f64),
                rug::Float::with_val(self.precision, -2.0 + slice * i as f64 + slice),
            );

            let imag_range = (
                rug::Float::with_val(self.precision, -2.0 + slice * i as f64),
                rug::Float::with_val(self.precision, -2.0 + slice * i as f64 + slice),
            );

            let precision = self.precision;
            let step_size = self.step_size.clone();
            let threshold = self.threshold.clone();
            workers.push(std::thread::spawn(move || {
                Self::worker(real_range, imag_range, precision, step_size, threshold)
            }));
        }

        workers.into_iter().for_each(|worker| match worker.join() {
            Ok(_) => (),
            Err(err) => log::error!("Failed to wait for worker: {err:?}"),
        });
    }

    fn name<'a>(&self) -> &'a str {
        "Mandelbrot set calculation"
    }
}

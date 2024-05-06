use std::{
    io::Write,
    sync::mpsc::{Receiver, Sender},
    thread::JoinHandle,
};

use rug::ops::Pow;

pub trait StressStrategy: Send {
    fn run(self);
    fn name<'a>(&self) -> &'a str;
}

pub struct LucasLehmerWorker {
    sender: Sender<u32>,
    join_handle: JoinHandle<()>,
}

pub struct LucasLehmer {
    workers: Vec<LucasLehmerWorker>,
    num_workers: usize,
}

impl LucasLehmer {
    pub fn new(num_workers: usize) -> Self {
        Self {
            workers: vec![],
            num_workers,
        }
    }

    fn worker(receiver: Receiver<u32>) {
        loop {
            if let Ok(power) = receiver.recv() {
                let mut s = rug::Integer::from(4);
                let m = rug::Integer::from(2).pow(power) - rug::Integer::from(1);
                for _ in 0..(power - 2) {
                    s = (s.square() - rug::Integer::from(2)).modulo(&m);
                }

                if s.is_zero() {
                    let _ = std::io::stdout()
                        .lock()
                        .write_fmt(format_args!("2^{} - 1 is prime\n", power));
                }
            }
        }
    }
}

pub struct PrimeIter {
    current: u32,
}

impl PrimeIter {
    pub fn new(start: u32) -> Self {
        Self { current: start }
    }
}

impl Default for PrimeIter {
    fn default() -> Self {
        Self::new(2)
    }
}

pub fn is_prime(n: u32) -> bool {
    let sqrt = f64::sqrt(n as f64).trunc() as u32;
    for i in 2..(sqrt + 1) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

impl Iterator for PrimeIter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > u32::MAX - 10_000 {
            None
        } else {
            self.current += 1;
            while !is_prime(self.current) {
                if self.current > u32::MAX - 10_000 {
                    return None;
                }
                self.current += 1;
            }
            Some(self.current)
        }
    }
}

impl StressStrategy for LucasLehmer {
    fn run(mut self) {
        for _ in 0..self.num_workers {
            let (tx, rx) = std::sync::mpsc::channel();
            self.workers.push(LucasLehmerWorker {
                sender: tx,
                join_handle: std::thread::spawn(move || {
                    Self::worker(rx);
                }),
            })
        }

        let mut prime_iter = PrimeIter::new(1000);
        'controller: loop {
            for worker in self.workers.iter() {
                match prime_iter.next() {
                    Some(power) => {
                        worker.sender.send(power).unwrap();
                    }
                    None => {
                        break 'controller;
                    }
                }
            }
        }

        self.workers.into_iter().for_each(|worker| {
            worker.join_handle.join().unwrap();
        })
    }

    fn name<'a>(&self) -> &'a str {
        "Lucas-Lehmer primality test"
    }
}

use std::{
    str::FromStr,
    sync::{mpsc::channel, Arc},
};

use rug::Complete;

use crate::stress::StressStrategy;

pub const RSA_1536: &str = "1847699703211741474306835620200164403018549338663410171471785774910651696711161249859337684305435744585616061544571794052229717732524660960646946071249623720442022269756756687378427562389508764678440933285157496578843415088475528298186726451339863364931908084671990431874381283363502795470282653297802934916155811881049844908319545009848393775227257052578591944993870073695755688436933812779613089230392569695253261620823676490316036551371447913932347169566988069";

pub struct RSA {
    num_threads: usize,
    modulus: Arc<rug::Integer>,
}

impl RSA {
    pub fn new(num_threads: usize, modulus: &str) -> Self {
        Self {
            num_threads,
            modulus: Arc::new(rug::Integer::from_str(modulus).expect("Invalid modulus")),
        }
    }
}

impl StressStrategy for RSA {
    fn run(&mut self) {
        let (tx, rx) = channel();
        let tx = Arc::new(tx);
        for thread_id in 0..self.num_threads {
            let n = Arc::clone(&self.modulus);
            let tx = Arc::clone(&tx);
            std::thread::spawn(move || {
                let mut a = n.sqrt_ref().complete();
                a += 1 + thread_id;
                let b: rug::Integer;

                loop {
                    let a2 = a.clone().square();
                    let b2 = (&a2 - &*n).complete();
                    if b2.is_perfect_square() {
                        b = b2.sqrt();
                        tx.send((a.clone(), b.clone())).unwrap();
                        break;
                    }
                    a += 1 + thread_id;
                }
            });
        }

        log::info!("Starting {} worker threads...", self.num_threads);
        log::info!("Computing the factors of n...\n");
        let (a, b) = rx.recv().unwrap();

        let p = (&a + &b).complete();
        let q = (&a - &b).complete();

        log::info!("----------!!DONE!!----------");
        log::info!("p = {p}");
        log::info!("q = {q}");
        log::info!("----------------------------");
    }

    fn name<'a>(&self) -> &'a str {
        "RSA-1536 factorization"
    }
}

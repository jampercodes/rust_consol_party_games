extern crate rand;
use rand::Rng;

pub fn start() {
    let mut rng = rand::thread_rng();
    let n: u32 = rng.gen_range(0..100);

    println!("Random number: {}", n);
}
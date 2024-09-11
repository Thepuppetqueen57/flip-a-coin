use rand::Rng;
use std::thread;
use std::time::Duration;

fn main() {
    let mut rng = rand::thread_rng();
    let coin_flip = rng.gen_bool(0.5); // 50% chance of true (heads) or false (tails)

    if coin_flip {
        println!("heads");
    } else {
        println!("tails");
    }

    thread::sleep(Duration::from_secs(3));
}
use gol::create_grid;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!(
            "Usage: cargo run [width] [height] [delay in ms]\n
Ex: cargo run 10 10 250"
        );
        return;
    }
    let (x, y, time) = (args[1].clone(), args[2].clone(), args[3].clone());
    create_grid(
        x.parse().unwrap(),
        y.parse().unwrap(),
        time.parse().unwrap(),
    );
}

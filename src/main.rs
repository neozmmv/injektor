use std::path::Path;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let start = Path::new(".");
    let dir: Vec<_> = std::fs::read_dir(start)
        .expect("Something went wrong reading the path.")
        .into_iter().collect();
    println!("ARGS: {:?}", args);
    println!("CURRENT DIR: {:?}", dir);
}

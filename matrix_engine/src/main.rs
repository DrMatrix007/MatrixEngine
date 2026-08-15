use std::env::current_dir;

fn main() {
    println!("Hello, world! {}", current_dir().unwrap().display());
}

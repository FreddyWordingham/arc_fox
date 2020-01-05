/// Main testing function.
use arc::{args, form};

form!(Parameters, num_threads: usize);

pub fn main() {
    println!("Hello world!");

    args!(_bin_path: String);
}

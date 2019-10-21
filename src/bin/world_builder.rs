//! World building example.

use arc::{util::{start_up, get_args, title}};
use std::path::Path;

fn main() {
    // Title.
    title("World Builder");

    // Start up.
    let (_cwd, _out_dir) = start_up(&Path::new("cwd"), &Path::new("out"));
    let args = get_args(vec!["<manifest.json>".to_string()]);
    let _input_file_path = &args[1];

    // Load manifest.
    // let man: Matcher = load(Path::new(input_file_path));
    // println!("Manifest:\n{:#?}", man);
}

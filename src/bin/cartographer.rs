//! Cartographer binary.
//! Creates a data cube mapping materials within a volume.

use arc::print;

fn main() {
    title();
}

fn title() {
    print::title("PTFE");
    colog::init();
}

fn start_up() -> (Vec<String>, PathBuf, PathBuf) {
    print::section("Start Up");

    let args = get_args(vec![]);
    for i in 0..args.len() {
        report!(args[i], (format!("args[{}]", i)));
    }

    let cwd = init::cwd("ptfe");
    report!(cwd.display(), "cwd");

    let out = init::output();
    report!(out.display(), "out");

    (args, cwd, out)
}

//! Main function.

use arc::util::{banner, exec};
use colog;

fn main() {
    colog::init();
    banner::title(&exec::name());
}

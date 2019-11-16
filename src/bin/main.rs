//! Main example function showing main capabilities.

use arc::{print::term::title, util::exec};

fn main() {
    title(&exec::name());
    colog::init();
}

//! Cartographer testing binary.

use arc::{
    args,
    file::io::{map, Load},
    report,
    sci::chem::{Reaction, ReactionBuilder, Species, SpeciesBuilder, State, StateBuilder},
    util::{
        dirs::init::io_dirs,
        info::exec,
        pb::Bar,
        print::term::{section, title},
    },
};
use attr_mac::form;
use colog;
use log::info;

#[form]
struct Parameters {
    num_threads: usize,
}

pub fn main() {
    colog::init();
    title(&exec::name());

    info!("Hello world!");
}

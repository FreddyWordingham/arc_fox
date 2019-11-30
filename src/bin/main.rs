//! Main example function demonstrating core capabilities.

use arc::{
    args,
    file::io::Load,
    form, report,
    sci::chem::ReactionBuilder,
    util::{
        dirs::init::io_dirs,
        info::exec,
        print::term::{section, title},
    },
    world::mat::InterfaceBuilder,
};
use log::info;
use std::path::Path;

form!(Parameters,
    reactions: Vec<String>;
    interfaces: Vec<String>
);

fn main() {
    title(&exec::name());
    colog::init();

    section("Initialisation");
    args!(
        _bin_path: String;
        form_path: String
    );
    let form_path = Path::new(&form_path);
    let (in_dir, out_dir) = io_dirs(None, None);

    section("Input");
    report!("Input dir", in_dir.display());
    report!(
        "Loading parameters from file",
        in_dir.join(form_path).display()
    );
    let form = Parameters::load(&in_dir.join(form_path));
    let _reactions: Vec<_> = form
        .reactions
        .iter()
        .map(|name| {
            let path = in_dir.join(format!("reactions/{}.json", name));
            info!("Loading reaction: {}", name);
            ReactionBuilder::load(&path)
        })
        .collect();
    let _interfaces: Vec<_> = form
        .interfaces
        .iter()
        .map(|name| {
            let path = in_dir.join(format!("interfaces/{}.json", name));
            info!("Loading interface: {}", name);
            InterfaceBuilder::load(&path)
        })
        .collect();

    section("Output");
    report!("Output dir", out_dir.display());

    section("Finished");
}

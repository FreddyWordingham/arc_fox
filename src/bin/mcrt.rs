//! MCRT test binary.

use arc::{
    args,
    file::{Loadable, Saveable},
    form::Entity as EntityForm,
    form::Mcrt,
    geom::{Aabb, Mesh},
    init::io_dirs,
    print, report,
    util::bin_name,
    world::{find_by_id, Entity, Material},
};
use contracts::pre;
use log::info;
use nalgebra::Point3;
use std::path::Path;

fn main() {
    title();
    args!(_bin_path: String, form_path: String);
    let form_path = Path::new(&form_path);
    let (in_dir, out_dir) = io_dirs(None, None);

    print::section("Input");
    report!("Input directory", in_dir.display());
    info!("Loading form: {}", form_path.display());
    let form = Mcrt::example();
    form.save(&in_dir.join("example.json"));

    print::section("Initialisation");
    let res = form.res();
    report!("Grid resolution", res);
    report!("Total cells", res.total());

    let dom = Aabb::new_centred(&Point3::origin(), form.half_widths());
    report!("X-width", dom.widths().x, "m");
    report!("Y-width", dom.widths().y, "m");
    report!("Z-width", dom.widths().z, "m");
    report!("Volume", dom.vol(), "m^3");

    let mats = load_mats(&in_dir.join("mats"), form.ents());
    let _ents = load_ents(&in_dir.join("meshes"), form.ents(), &mats);

    // let uni = Universe::

    print::section("Simulation");

    print::section("Post-Processing");

    print::section("Output");
    report!("Output directory", out_dir.display());

    print::section("End");
}

fn title() {
    print::title(&bin_name());
    colog::init();
}

#[pre(dir.is_dir())]
#[pre(!ents.is_empty())]
fn load_mats(dir: &Path, ents: &Vec<EntityForm>) -> Vec<Material> {
    let ids = get_mat_ids(ents);

    let mut mats = Vec::with_capacity(ids.len());
    for id in ids.iter() {
        info!("Loading material: {}", id);

        let path = dir.join(format!("{}.json", id));
        mats.push(Material::load(&path));
    }
    info!("Loaded {} materials total.", mats.len());

    mats
}

#[pre(!ents.is_empty())]
#[post(!ret.is_empty())]
fn get_mat_ids(ents: &Vec<EntityForm>) -> Vec<String> {
    let mut ids: Vec<String> = Vec::new();

    for ent in ents.iter() {
        ids.push(ent.in_mat.clone());
        ids.push(ent.out_mat.clone());
    }

    ids.sort();
    ids.dedup();

    ids
}

#[pre(dir.is_dir())]
#[pre(!ent_forms.is_empty())]
fn load_ents<'a>(
    dir: &Path,
    ent_forms: &Vec<EntityForm>,
    mats: &'a Vec<Material>,
) -> Vec<Entity<'a>> {
    let mut ents = Vec::with_capacity(ent_forms.len());
    for ent in ent_forms.iter() {
        info!("Loading entity: {}", ent.id);

        let path = dir.join(format!("{}.obj", ent.mesh));
        let mut mesh = Mesh::load(&path);

        ents.push(Entity::new(
            ent.id.clone(),
            mesh,
            find_by_id(mats, &ent.in_mat),
            find_by_id(mats, &ent.out_mat),
        ))
    }
    info!("Loaded {} entities total.", ents.len());

    ents
}

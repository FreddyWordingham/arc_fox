//! Format functions.

use crate::{
    chem::{Rate, Reaction},
    dom::Grid,
    world::{InterMap, MatMap, MolMap, ReactMap, Universe},
};
use std::fmt::Write;

/// Print a formatted overview of a given universe
pub fn universe(uni: &Universe) -> String {
    let mut fmt = String::new();

    write!(fmt, "Grid:\n{}", grid(uni.grid())).unwrap();
    write!(fmt, "Molecules:\n{}", mol_map(uni.mol_map())).unwrap();
    write!(
        fmt,
        "Reactions:\n{}",
        react_map(uni.react_map(), uni.mol_map())
    )
    .unwrap();
    write!(fmt, "Materials:\n{}", mat_map(uni.mat_map())).unwrap();
    write!(fmt, "Interfaces:\n{}", inter_map(uni.inter_map())).unwrap();

    write!(fmt, "{:<32}: {}s", "Age", uni.age()).unwrap();

    fmt
}

/// Print a formatted overview of a given grid.
fn grid(grid: &Grid) -> String {
    let mut fmt = String::new();

    writeln!(
        fmt,
        "\t{:<28}: {}m x {}m x {}m",
        "Dimensions",
        grid.dom().widths().x,
        grid.dom().widths().y,
        grid.dom().widths().z
    )
    .unwrap();
    writeln!(fmt, "\t{:<28}: {}m^3", "Total volume", grid.dom().vol()).unwrap();

    let res = grid.res();
    writeln!(
        fmt,
        "\t{:<28}: {} x {} x {}",
        "Resolution",
        res.x(),
        res.y(),
        res.z()
    )
    .unwrap();
    writeln!(fmt, "\t{:<28}: {}", "Total cells", res.total()).unwrap();
    writeln!(
        fmt,
        "\t{:<28}: {}m^3",
        "Cell volume",
        grid.dom().vol() / res.total() as f64
    )
    .unwrap();

    fmt
}

/// Print a formatted overview of a given molecule-map.
fn mol_map(mol_map: &MolMap) -> String {
    let mut fmt = String::new();

    for (name, mol) in mol_map.iter() {
        if let Some(rad) = mol.rad() {
            writeln!(fmt, "\t{:<28}: {}nm", name, rad * 1.0e9).unwrap();
        } else {
            writeln!(fmt, "\t{:<28}: unsized", name).unwrap();
        }
    }

    fmt
}

/// Print a formatted overview of a given reaction-map.
fn react_map(react_map: &ReactMap, mol_map: &MolMap) -> String {
    let mut fmt = String::new();

    let mol_names: Vec<&str> = mol_map.iter().map(|(n, _m)| n.as_str()).collect();

    for (name, react) in react_map.iter() {
        writeln!(fmt, "\t{:<28}: {}", name, reaction(react, &mol_names)).unwrap();
    }

    fmt
}

/// Print a formatted overview of a given reaction.
fn reaction(reaction: &Reaction, mol_names: &Vec<&str>) -> String {
    let mut fmt = String::new();

    let (i, s) = reaction.reactants()[0];
    write!(fmt, "{}{}", s, mol_names[i]).unwrap();
    for (i, s) in reaction.reactants().iter().skip(1) {
        write!(fmt, " + {}{}", s, mol_names[*i]).unwrap();
    }

    write!(fmt, " -> ").unwrap();

    let (i, s) = reaction.products()[0];
    write!(fmt, "{}{}", s, mol_names[i]).unwrap();
    for (i, s) in reaction.products().iter().skip(1) {
        write!(fmt, " + {}{}", s, mol_names[*i]).unwrap();
    }

    let mut fmt = format!("{:<32} at ", fmt);

    match reaction.rate() {
        Rate::ZerothOrder(k) => write!(fmt, "{}", k).unwrap(),
        Rate::FirstOrder(k, c_id) => write!(fmt, "{}[{}]", k, mol_names[*c_id]).unwrap(),
        Rate::SecondOrder(k, c_id_0, c_id_1) => {
            write!(fmt, "{}[{}][{}]", k, mol_names[*c_id_0], mol_names[*c_id_1]).unwrap()
        }
    }

    write!(fmt, " mol/(m^3 s)").unwrap();

    fmt
}

/// Print a formatted overview of a given material-map.
fn mat_map(mat_map: &MatMap) -> String {
    let mut fmt = String::new();

    for (name, mat) in mat_map.iter() {
        if let Some(visc) = mat.visc() {
            writeln!(fmt, "\t{:<28}: {}Pa s", name, visc).unwrap();
        } else {
            writeln!(fmt, "\t{:<28}: Non-diffusive", name).unwrap();
        }
    }

    fmt
}

/// Print a formatted overview of a given interface-map.
fn inter_map(inter_map: &InterMap) -> String {
    let mut fmt = String::new();

    for (name, inter) in inter_map.iter() {
        writeln!(
            fmt,
            "\t{:<28}: {} triangles",
            name,
            inter.mesh().tris().len()
        )
        .unwrap();
    }

    fmt
}

//! Format functions.

use crate::{
    chem::{Rate, Reaction},
    dom::Grid,
    world::{MolMap, ReactMap, Universe},
};
use std::fmt::Write;

/// Print a formatted overview of a given universe
pub fn universe(uni: &Universe) -> String {
    let mut fmt = String::new();

    writeln!(fmt, "Grid:\n{}", grid(uni.grid())).unwrap();
    writeln!(fmt, "Molecules:\n{}", mol_map(uni.mol_map())).unwrap();
    writeln!(
        fmt,
        "Reactions:\n{}",
        react_map(uni.react_map(), uni.mol_map())
    )
    .unwrap();

    fmt
}

/// Print a formatted overview of a given grid.
pub fn grid(grid: &Grid) -> String {
    let mut fmt = String::new();

    writeln!(
        fmt,
        "{:<32}: {}m x {}m x {}m",
        "Dimensions",
        grid.dom().widths().x,
        grid.dom().widths().y,
        grid.dom().widths().z
    )
    .unwrap();
    writeln!(fmt, "{:<32}: {}m^3", "Total volume", grid.dom().vol()).unwrap();

    let res = grid.res();
    writeln!(
        fmt,
        "{:<32}: {} x {} x {}",
        "Resolution",
        res.x(),
        res.y(),
        res.z()
    )
    .unwrap();
    writeln!(fmt, "{:<32}: {}", "Total cells", res.total()).unwrap();
    writeln!(
        fmt,
        "{:<32}: {}m^3",
        "Cell volume",
        grid.dom().vol() / res.total() as f64
    )
    .unwrap();

    fmt
}

/// Print a formatted overview of a given molecule-map.
pub fn mol_map(mol_map: &MolMap) -> String {
    let mut fmt = String::new();

    for (name, mol) in mol_map.iter() {
        if let Some(rad) = mol.rad() {
            writeln!(fmt, "{:<32}: {}nm", name, rad * 1.0e9).unwrap();
        } else {
            writeln!(fmt, "{:<32}: unsized", name).unwrap();
        }
    }

    fmt
}

/// Print a formatted overview of a given reaction-map.
pub fn react_map(react_map: &ReactMap, mol_map: &MolMap) -> String {
    let mut fmt = String::new();

    let mol_names: Vec<&str> = mol_map.iter().map(|(n, _m)| n.as_str()).collect();

    for (name, react) in react_map.iter() {
        writeln!(fmt, "{:<32}: {}", name, reaction(react, &mol_names)).unwrap();
    }

    fmt
}

/// Print a formatted overview of a given reaction.
pub fn reaction(reaction: &Reaction, mol_names: &Vec<&str>) -> String {
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

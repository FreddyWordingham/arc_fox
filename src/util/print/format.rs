//! Formatting functions.

use crate::{
    sci::chem::{Rate, Reaction, Species},
    world::{
        mat::{Interface, Material},
        parts::Named,
        Universe,
    },
};
use log::info;
use std::fmt::Write;

/// Print a universe.
pub fn universe(universe: &Universe) {
    info!(
        "\n{:<16}{:<16}\n{}",
        "Species",
        "Radius",
        species(universe.species())
    );
    info!(
        "\n{:<16}{:<32}{:<16}\n{}",
        "Reaction",
        "Formula",
        "Rate",
        reactions(universe.reactions(), universe.species())
    );
    info!(
        "\n{:<16}{:<16}{:<16}\n{}",
        "Materials",
        "Viscosity",
        "Multiplier",
        materials(universe.materials())
    );
    info!(
        "\n{:<16}{:<16}{:<16}\n{}",
        "Interfaces",
        "Inside",
        "Outside",
        interfaces(universe.interfaces())
    );
}

/// Print a list of reactions.
pub fn species(species: &[Species]) -> String {
    let mut fmt = String::new();

    for spec in species {
        let name = spec.name();
        let size = if let Some(rad) = spec.rad() {
            format!("{}nm", rad * 1.0e9)
        } else {
            "unsized".to_string()
        };

        writeln!(fmt, "{:<16}{:<16}", name, size).unwrap();
    }
    fmt.pop();

    fmt
}

/// Print a list of reactions.
pub fn reactions(reactions: &[Reaction], species: &[Species]) -> String {
    let mut fmt = String::new();

    for reaction in reactions {
        let name = reaction.name();

        let mut reactants = String::new();
        let (id, s) = reaction.reactants()[0];
        write!(reactants, "{}{}", s, species[id].name()).unwrap();
        for (id, s) in reaction.reactants().iter().skip(1) {
            write!(reactants, " + {}{}", s, species[*id].name()).unwrap();
        }

        let mut products = String::new();
        let (id, s) = reaction.products()[0];
        write!(products, "{}{}", s, species[id].name()).unwrap();
        for (id, s) in reaction.products().iter().skip(1) {
            write!(products, " + {}{}", s, species[*id].name()).unwrap();
        }

        let rate = rate(reaction.rate(), species);

        writeln!(
            fmt,
            "{:<16}{:<32}{:<16}",
            name,
            format!("{} -> {}", reactants, products),
            rate,
        )
        .unwrap();
    }
    fmt.pop();

    fmt
}

/// Print the formatted rate as a string.
pub fn rate(rate: &Rate, species: &[Species]) -> String {
    let mut fmt = String::new();

    match rate {
        Rate::Zeroth(k) => write!(fmt, "{}", k).unwrap(),
        Rate::First(k, a) => write!(fmt, "{}[{}]", k, species[*a].name()).unwrap(),
        Rate::Second(k, a, b) => {
            write!(fmt, "{}[{}][{}]", k, species[*a].name(), species[*b].name()).unwrap()
        }
        Rate::Third(k, a, b, c) => write!(
            fmt,
            "{}[{}][{}][{}]",
            k,
            species[*a].name(),
            species[*b].name(),
            species[*c].name()
        )
        .unwrap(),
        Rate::Poly(k, is) => {
            write!(fmt, "{}", k).unwrap();
            for i in is {
                write!(fmt, "[{}]", species[*i].name()).unwrap();
            }
        }
    }

    fmt
}

/// Print a list of materials.
pub fn materials(materials: &[Material]) -> String {
    let mut fmt = String::new();

    for material in materials {
        let name = material.name();
        let visc = if let Some(visc) = material.visc() {
            format!("{}PaS", visc)
        } else {
            "impermeable".to_string()
        };
        let mult = material.reaction_multiplier();

        writeln!(fmt, "{:<16}{:<16}{:<16}", name, visc, mult).unwrap();
    }
    fmt.pop();

    fmt
}

/// Print a list of interfaces.
pub fn interfaces(interfaces: &[Interface]) -> String {
    let mut fmt = String::new();

    for interface in interfaces {
        let name = interface.name();
        let in_mat = interface.in_mat().name();
        let out_mat = interface.out_mat().name();

        writeln!(fmt, "{:<16}{:<16}{:<16}", name, in_mat, out_mat,).unwrap();
    }
    fmt.pop();

    fmt
}

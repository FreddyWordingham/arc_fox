//! Run in serial functions.

use super::Statemap;
use crate::{
    base::Resolution,
    file::Save,
    world::{MolMap, ReactMap},
};
use contracts::pre;
use nalgebra::Vector3;
use ndarray::{Array1, Array3};
use std::{f64::INFINITY, path::Path};

/// Run an evolution simulation in serial.
#[pre(out_dir.is_dir())]
#[pre(sim_time > 0.0)]
#[pre(dump_time > 0.0)]
#[pre(max_dt > 0.0)]
pub fn run(
    out_dir: &Path,
    sim_time: f64,
    dump_time: f64,
    max_dt: f64,
    res: &Resolution,
    cell_size: &Vector3<f64>,
    diff_coeffs: &Array3<Option<Array1<Option<f64>>>>,
    mol_map: &MolMap,
    react_map: &ReactMap,
    statemap: &mut Statemap,
    reaction_multipliers: &Array3<f64>,
) {
    let mut time = 0.0;
    let mut time_since_dump = INFINITY;
    while time < sim_time {
        let time_to_dump = {
            let t = dump_time - time_since_dump;

            if t <= 0.0 {
                dump_time
            } else {
                t
            }
        };
        let dt = max_dt.min(time_to_dump);

        // println!("Time is {}", time);

        let diff_deltas = diff_deltas(res, cell_size, diff_coeffs, mol_map, statemap);

        let react_deltas = react_deltas(res, mol_map, react_map, statemap, reaction_multipliers);

        let mut min_react_delta_time = None;
        for (ds, state) in react_deltas.iter().zip(statemap.states.iter()) {
            for (d, c) in ds.iter().zip(state.concs().iter()) {
                if *c > 0.0 {
                    let min_dt = -c / (d * 10.0);
                    // if (-min_dt * d) >= (c / 2.0) {
                    //     panic!("Going to fail...");
                    // }
                    if min_dt > 0.0 {
                        if min_react_delta_time.is_none() || min_dt < min_react_delta_time.unwrap()
                        {
                            min_react_delta_time = Some(min_dt);
                        }
                    }
                }
            }
        }
        let min_react_delta_time = min_react_delta_time.unwrap();

        // let dt = (dt.min(min_react_delta_time)).max(0.001);
        let dt = dt.min(min_react_delta_time);
        if dt < 0.0 {
            panic!("Negative time delta.");
        }
        println!("Time is {} -> {}", time, dt);

        let deltas = diff_deltas + react_deltas;
        for (state, delta) in statemap.states.iter_mut().zip(deltas.iter()) {
            for (e, (conc, d)) in state.mut_concs().iter_mut().zip(delta.iter()).enumerate() {
                *conc += d * dt;
                if *conc < 0.0 {
                    panic!("Negative concentration > ! {}", e);
                }
            }
        }

        time += dt;
        time_since_dump += dt;
        if time_since_dump >= dump_time {
            println!("Dumping at: {}", time);

            statemap
                .mol_concs(mol_map)
                .save(&out_dir.join(format!("{}.nc", (time * 1000.0) as i64)));

            time_since_dump = 0.0;
        }
    }
}

/// Calculate the diffusion deltas.
fn diff_deltas(
    res: &Resolution,
    cell_size: &Vector3<f64>,
    diff_coeffs: &Array3<Option<Array1<Option<f64>>>>,
    mol_map: &MolMap,
    statemap: &Statemap,
) -> Array3<Array1<f64>> {
    let mut deltas: Array3<Array1<f64>> =
        Array3::from_elem(*res.arr(), Array1::zeros(mol_map.len()));

    for index in res {
        if let Some(diff_coeffs) = &diff_coeffs[*index.arr()] {
            let prev_x = res.adjacent(&index, [-1, 0, 0]).unwrap_or(index.clone());
            let next_x = res.adjacent(&index, [1, 0, 0]).unwrap_or(index.clone());
            let prev_y = res.adjacent(&index, [0, -1, 0]).unwrap_or(index.clone());
            let next_y = res.adjacent(&index, [0, 1, 0]).unwrap_or(index.clone());
            let prev_z = res.adjacent(&index, [0, 0, -1]).unwrap_or(index.clone());
            let next_z = res.adjacent(&index, [0, 0, 1]).unwrap_or(index.clone());

            for (i, (c, d)) in diff_coeffs
                .iter()
                .zip(deltas[*index.arr()].iter_mut())
                .enumerate()
            {
                if let Some(c) = c {
                    let own_conc = statemap.states[*index.arr()].concs()[i];
                    let prev_x_conc = statemap.states[*prev_x.arr()].concs()[i];
                    let next_x_conc = statemap.states[*next_x.arr()].concs()[i];
                    let prev_y_conc = statemap.states[*prev_y.arr()].concs()[i];
                    let next_y_conc = statemap.states[*next_y.arr()].concs()[i];
                    let prev_z_conc = statemap.states[*prev_z.arr()].concs()[i];
                    let next_z_conc = statemap.states[*next_z.arr()].concs()[i];

                    *d = statemap.states[*index.arr()].sources()[i];
                    *d += *c
                        * (((prev_x_conc - (2.0 * own_conc) + next_x_conc) / cell_size.x.powi(2))
                            + ((prev_y_conc - (2.0 * own_conc) + next_y_conc)
                                / cell_size.y.powi(2))
                            + ((prev_z_conc - (2.0 * own_conc) + next_z_conc)
                                / cell_size.z.powi(2)));
                }
            }
        }
    }

    deltas
}

/// Calculate the reaction deltas.
fn react_deltas(
    res: &Resolution,
    mol_map: &MolMap,
    react_map: &ReactMap,
    statemap: &Statemap,
    reaction_multipliers: &Array3<f64>,
) -> Array3<Array1<f64>> {
    let mut deltas: Array3<Array1<f64>> =
        Array3::from_elem(*res.arr(), Array1::zeros(mol_map.len()));

    for index in res {
        let concs = statemap.states[*index.arr()].concs();
        let ds = &mut deltas[*index.arr()];
        let m = reaction_multipliers[*index.arr()];

        for (_name, reaction) in react_map.iter() {
            let rate = reaction.rate().res(&concs);

            for (i, s) in reaction.reactants().iter() {
                ds[*i] += rate * m * *s as f64;
            }
            for (i, s) in reaction.products().iter() {
                ds[*i] += -rate * m * *s as f64;
            }
        }
    }

    deltas
}

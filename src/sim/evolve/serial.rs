//! Run in serial functions.

use super::Statemap;
use crate::{base::Resolution, file::Save, world::MolMap};
use contracts::pre;
use nalgebra::Vector3;
use ndarray::{Array1, Array3};
use std::path::Path;

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
    statemap: &mut Statemap,
) {
    let mut time = 0.0;
    let mut time_since_dump = 0.0;
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

        println!("Time is {}", time);

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
                            * (((prev_x_conc - (2.0 * own_conc) + next_x_conc)
                                / cell_size.x.powi(2))
                                + ((prev_y_conc - (2.0 * own_conc) + next_y_conc)
                                    / cell_size.y.powi(2))
                                + ((prev_z_conc - (2.0 * own_conc) + next_z_conc)
                                    / cell_size.z.powi(2)));
                    }
                }
            }
        }

        for (state, delta) in statemap.states.iter_mut().zip(deltas.iter()) {
            for (conc, d) in state.mut_concs().iter_mut().zip(delta.iter()) {
                *conc += d * dt;
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

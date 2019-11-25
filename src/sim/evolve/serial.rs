//! Run in serial functions.

use super::Statemap;
use contracts::pre;

/// Run an evolution simulation in serial.
#[pre(sim_time > 0.0)]
#[pre(dump_time > 0.0)]
pub fn run(sim_time: f64, dump_time: f64, statemap: &mut Statemap) {
    
}

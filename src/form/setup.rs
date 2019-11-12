use crate::json;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct Setup {
    res: [usize; 3],
}

json!(Setup);

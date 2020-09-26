use serde::{Serialize, Deserialize};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug)]
pub struct NewActivity {
    name: String,
    description: Option<String>,
    start_date: String,
    end_date: String,
    category: String,
    status: String,
}
use serde::{Deserialize, Serialize};

use crate::{filter::Filter, tasks::Task};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedState {
    inpu_value: String,
    filter: Filter,
    tasks: Vec<Task>,
}

#[derive(Debug, Clone)]
pub enum LoadError {
    File,
    Format,
}

#[derive(Debug, Clone)]
pub enum SaveError {
    File,
    Write,
    Format,
}

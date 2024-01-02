use std::{env::current_dir, path::PathBuf, time::Duration};

use directories_next::ProjectDirs;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};

use crate::tasks::{Filter, Task};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedState {
    pub input_value: String,
    pub filter: Filter,
    pub tasks: Vec<Task>,
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

impl SavedState {
    pub fn path() -> PathBuf {
        let mut path = if let Some(project_dirs) = ProjectDirs::from("rs", "Iced", "Todos") {
            project_dirs.data_dir().into()
        } else {
            current_dir().unwrap_or_default()
        };

        path.push("todos.json");

        path
    }

    pub async fn load() -> Result<Self, LoadError> {
        use async_std::prelude::*;

        let mut contents = String::new();

        let mut file = async_std::fs::File::open(Self::path())
            .await
            .map_err(|_| LoadError::File)?;

        file.read_to_string(&mut contents)
            .await
            .map_err(|_| LoadError::File)?;

        from_str(&contents).map_err(|_| LoadError::Format)
    }

    pub async fn save(self) -> Result<(), SaveError> {
        use async_std::prelude::*;

        let json = to_string_pretty(&self).map_err(|_| SaveError::Format)?;

        let path = Self::path();

        if let Some(dir) = path.parent() {
            async_std::fs::create_dir_all(dir)
                .await
                .map_err(|_| SaveError::File)?;
        }

        {
            let mut file = async_std::fs::File::create(path)
                .await
                .map_err(|_| SaveError::File)?;

            file.write_all(json.as_bytes())
                .await
                .map_err(|_| SaveError::Write)?;
        }

        async_std::task::sleep(Duration::from_secs(2)).await;

        Ok(())
    }
}

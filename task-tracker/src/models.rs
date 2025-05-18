use crate::util::cur_time_ms_u128;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::From;
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
pub struct TaskJson(pub HashMap<u32, Task>);

impl TaskJson {
    fn create_default_file(path: &str) {
        fs::write(path, "{}").expect("Failed writing file.");
    }

    fn write_to_file<C: AsRef<[u8]>>(path: &str, content: C) {
        fs::write(path, content).expect("Failed writing file.");
    }

    pub fn from_file(path: &str) -> Self {
        let content = match fs::read_to_string(path) {
            Ok(v) => v,
            Err(e) => {
                eprintln!(
                    "Got error reading {}: {:?}\nWill be creating default file.",
                    path, e
                );
                Self::create_default_file(path);
                String::from("{}")
            }
        };

        match serde_json::from_str(&content) {
            Ok(v) => v,
            Err(e) => {
                eprintln!(
                    "Error deserializing {0}: {1:?}\nWill be resetting the file. Old file content will be moved to: _{0}",
                    path, e
                );
                Self::write_to_file(&format!("_{}", path), &content);
                Self::create_default_file(path);
                Self(HashMap::new())
            }
        }
    }

    pub fn save_to_file(&self, path: &str) {
        Self::write_to_file(path, serde_json::to_vec(&self.0).unwrap());
    }

    // stinky
    fn get_next_id(&self) -> u32 {
        self.0.keys().max().map_or(1, |max| max + 1)
    }

    pub fn insert_task(&mut self, task: Task) -> u32 {
        let id = self.get_next_id();
        self.0.insert(id, task);
        id
    }

    pub fn remove_task(&mut self, id: u32) -> Option<Task> {
        self.0.remove(&id)
    }

    pub fn get_task(&self, id: u32) -> Option<&Task> {
        self.0.get(&id)
    }

    pub fn get_id_description_pair(&self) -> Vec<(u32, &str)> {
        let mut result = Vec::with_capacity(self.0.len());
        for (k, v) in self.0.iter() {
            result.push((*k, v.description.as_str()))
        }
        result
    }

    pub fn get_id_description_pair_filtered(&self, status: &str) -> Vec<(u32, &str)> {
        let mut result = Vec::with_capacity(self.0.len());
        for (k, v) in self.0.iter() {
            if v.status == status {
                result.push((*k, v.description.as_str()))
            }
        }
        result
    }

    pub fn update_task(
        &mut self,
        id: u32,
        description: Option<&str>,
        status: Option<&str>,
    ) -> Option<()> {
        let Some(task) = self.0.get_mut(&id) else {
            return None;
        };

        match (description, status) {
            (Some(new_description), _) if !(task.description == new_description) => {
                task.description = new_description.to_string();
            }
            (_, Some(new_status)) if !(task.status == new_status) => {
                task.status = new_status.to_string();
            }
            // this is unreachable because well, just look at how the function is being called.
            _ => unreachable!(),
        }

        task.last_updated = cur_time_ms_u128();

        Some(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub description: String,
    pub status: String,
    pub created_at: u128,
    pub last_updated: u128,
}

impl Task {
    pub fn new(description: &str) -> Self {
        let cur_time = cur_time_ms_u128();
        Self {
            description: description.to_string(),
            status: "todo".to_string(),
            created_at: cur_time,
            last_updated: cur_time,
        }
    }
}

// #[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
// pub enum TaskStatus {
//     #[default]
//     Todo,
//     InProgress,
//     Done,
// }

// impl From<&str> for TaskStatus {
//     fn from(value: &str) -> Self {
//         match value.to_lowercase() {
//             "todo" => Self::Todo,

//         }
//     }
// }

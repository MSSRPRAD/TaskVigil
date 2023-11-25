use std::io::{self, BufReader, BufWriter, Read, Write};
use std::fs::{File, OpenOptions, read_dir, create_dir_all};
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use home;

const PROJECTS_DIR: &str = ".taskvigil/projects";

/// Enum representing different types for a task.
#[derive(Debug, Deserialize, Serialize)]
enum Type {
    Urgent,
    TimePass,
    Casual,
    TimeWaste,
    Recreation,
}

/// Represents a session within a task.
#[derive(Debug, Deserialize, Serialize)]
pub struct Session {
    /// The start time of the session.
    start_time: String,
    /// The end time of the session.
    end_time: String,
    /// Optional state changes during the session.
    state_changes: Option<Vec<(Type, Type)>>,
}

/// Represents a task within a project.
#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    /// The name of the task.
    pub name: String,
    /// Sessions associated with the task.
    pub sessions: Vec<Session>,
    /// Indicates whether the task is finished.
    pub finished: bool,
}

/// Represents a project with multiple tasks.
#[derive(Debug, Deserialize, Serialize)]
pub struct Project {
    /// The name of the project.
    pub name: String,
    /// Tasks associated with the project.
    pub tasks: Vec<Task>,
    /// Tags associated with Project.
    pub tags: Vec<String>,
}

impl Project {
    pub fn new(name: String, tags: Option<Vec<String>>) -> Self {
        Project {
            name: name,
            tasks: Vec::new(),
            tags: match tags {
                None => vec!["Misc".to_string()],
                Some(tags) => tags,
            }
        }
    }
}
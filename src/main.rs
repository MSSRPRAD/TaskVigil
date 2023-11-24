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
struct Session {
    /// The start time of the session.
    start_time: String,
    /// The end time of the session.
    end_time: String,
    /// Optional state changes during the session.
    state_changes: Option<Vec<(Type, Type)>>,
}

/// Represents a task within a project.
#[derive(Debug, Deserialize, Serialize)]
struct Task {
    /// The name of the task.
    name: String,
    /// Sessions associated with the task.
    sessions: Vec<Session>,
    /// Indicates whether the task is finished.
    finished: bool,
}

/// Represents a project with multiple tasks.
#[derive(Debug, Deserialize, Serialize)]
struct Project {
    /// The name of the project.
    name: String,
    /// Tasks associated with the project.
    tasks: Vec<Task>,
    /// Tags associated with Project.
    tags: Vec<String>,
}

impl Project {
    fn new(name: String, tags: Option<Vec<String>>) -> Self {
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

/// Reads project data from a file.
fn read_project_data(file_path: &str) -> Result<Project, io::Error> {
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening file {}: {}", file_path, err);
            return Err(err);
        }
    };

    let reader = BufReader::new(file);
    let mut content = String::new();
    if let Err(err) = reader.take(1000).read_to_string(&mut content) {
        eprintln!("Error reading file {}: {}", file_path, err);
        return Err(err);
    }

    let project: Project = match serde_yaml::from_str(&content) {
        Ok(project) => project,
        Err(err) => {
            eprintln!("Error parsing project data from {}: {}", file_path, err);
            return Err(io::Error::new(io::ErrorKind::Other, err));
        }
    };

    Ok(project)
}

/// Writes project data to a file.
fn write_project_data(file_path: &str, project: &Project) -> Result<(), io::Error> {
    let file = File::create(file_path)?;
    let writer = BufWriter::new(file);

    let project_str: &str = &serde_yaml::to_string(project).unwrap();
    let mut writer = BufWriter::new(writer);
    writer.write_all(project_str.as_bytes())?;
    Ok(())
}

/// Reads all projects from the specified directory.
fn read_all_projects(directory: &str) -> Vec<Project> {
    let mut projects = Vec::new();

    println!("REACHED HERE!");
    if let Ok(entries) = read_dir(directory) {
        println!("REACHED HERE!");
        for entry in entries {
            println!("ENTRY: {:?}", entry);
            if let Ok(entry) = entry {
                if entry.path().is_file() {
                    if let Ok(project) = read_project_data(entry.path().to_str().unwrap()) {
                        projects.push(project);
                    }
                }
            }
        }
    }

    projects
}


fn write_dummy_project(project_name: &str, tasks: Vec<Task>) -> Result<(), io::Error> {
    let project = Project {
        name: project_name.to_string(),
        tasks,
        tags: Vec::new(),
    };

    let projects_directory = format!("{}/{}", home::home_dir().unwrap().display(), PROJECTS_DIR);
    if !Path::new(&projects_directory).exists() {
        create_dir_all(&projects_directory)?;
    }

    let project_file_path = format!("{}/{}.yaml", projects_directory, project_name);
    let file = File::create(project_file_path)?;
    let writer = BufWriter::new(file);

    serde_yaml::to_writer(writer, &project).unwrap();

    Ok(())
}

fn main() {
    println!("{:?}", read_all_projects("/home/mpradyumna/.taskvigil/projects"));
}
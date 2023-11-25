use std::{
    fs::{self, File, read_dir},
    io::{self, BufReader, BufWriter, Read, Write},
    path::{Path, PathBuf},
};

use crate::structs::structs::*;
use lazy_static::lazy_static;

lazy_static! {
    static ref PROJECTS_DIR: PathBuf = {
        let home_dir = home::home_dir().unwrap_or_default();
        let projects_path = home_dir.join(".taskvigil").join("projects");
        projects_path
    };
}

/// Reads project data from a file.
pub fn read_project_data(project_name: &str) -> Result<Project, io::Error> {
    let file_path = PROJECTS_DIR.join(project_name).with_extension("yaml");

    let file = File::open(&file_path)?;
    let mut reader = BufReader::new(file);

    let mut content = String::new();
    reader.read_to_string(&mut content)?;

    let project: Project = serde_yaml::from_str(&content)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, format!("Error parsing project data: {}", err)))?;

    Ok(project)
}

/// Writes project data to a file.
pub fn write_project_data(project: &Project) -> Result<(), io::Error> {
    let file_path = PROJECTS_DIR.join(&project.name).with_extension("yaml");

    fs::create_dir_all(file_path.parent().unwrap())?;
    let file = File::create(&file_path)?;
    let mut writer = BufWriter::new(file);

    let project_str = serde_yaml::to_string(project).unwrap();
    writer.write_all(project_str.as_bytes())?;

    Ok(())
}

/// Reads all projects from the specified directory.
pub fn read_all_projects() -> Vec<Project> {
    let mut projects = Vec::new();

    if let Ok(entries) = read_dir(&*PROJECTS_DIR) {
        for entry in entries.flatten() {
            if entry.path().extension().unwrap_or_default() == "yaml" {
                println!("{:?}", entry.path());
                if let Ok(project) = read_project_data(entry.file_name().to_str().unwrap()) {
                    projects.push(project);
                }
            }
        }
    }

    projects
}
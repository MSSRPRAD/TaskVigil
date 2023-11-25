use std::{
    fs::{self, File, read_dir},
    io::{self, BufReader, BufWriter, Read, Write},
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH}, thread::sleep,
};
use chrono::{Local, DateTime, Duration, TimeZone, Timelike};
use crate::structs::structs::*;
use lazy_static::lazy_static;

const DATE_FORMAT_STR: &'static str = "%Y-%m-%d %H:%M:%S";

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

pub fn resume_task(project: &mut Project, task_name: &str) {
    if let Some(task) = project.tasks.iter_mut().find(|t| t.name == task_name) {
        if let Some(last_session) = task.sessions.last_mut() {
            if last_session.end_time.is_some() {
                // Start a new session only if the last one is finished
                task.sessions.push(Session {
                    start_time: chrono::Local::now().to_string(),
                    end_time: None,
                    state_changes: Some(vec![(Type::Misc, Type::TimePass)]),
                });
                println!("Resumed task: {}", task_name);
            } else {
                println!("Task is already in progress. Cannot resume.");
            }
        }
    } else {
        println!("Task not found: {}", task_name);
    }
}

pub fn finish_task(project: &mut Project, task_name: &str) {
    if let Some(task) = project.tasks.iter_mut().find(|t| t.name == task_name) {
        if let Some(last_session) = task.sessions.last_mut() {
            if last_session.end_time.is_none() {
                // Finish the task only if it's currently in progress
                last_session.end_time = Some(chrono::Local::now().to_rfc2822().to_string());
                task.finished = true;
                println!("Finished task: {}", task_name);
            } else {
                println!("Task is already finished. Cannot finish again.");
            }
        }
    } else {
        println!("Task not found: {}", task_name);
    }
}

pub fn print_status(project: &Project) {
    sleep(std::time::Duration::from_secs(2));
    println!("Project Status: {}", project.name);
    for task in &project.tasks {
        println!("Task: {}", task.name);
        for session in &task.sessions {
            let start_time = chrono::DateTime::parse_from_rfc2822(&session.start_time).unwrap();
            let end_time =  chrono::Local::now();
            let duration = end_time.signed_duration_since(start_time);
            println!("  - Start Time    : {}", start_time);
            println!("  - End Time      : {}", end_time);
            println!("  - Duration      : {} seconds", duration.abs().num_seconds());
            if let Some(state_changes) = &session.state_changes {
                for (from, to) in state_changes {
                    println!("  - State Change  : {:?} -> {:?}", from, to);
                }
            }
        }
        if task.finished {
            println!("  - Task is Finished");
        } else {
            println!("  - Task is In Progress");
        }
    }
}
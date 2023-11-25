use TaskVigil::structs::{structs::*, utils::print_status};
use clap::{Parser};
use std::time::{SystemTime, UNIX_EPOCH};

/// Simple program to manage projects and tasks.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Project Name
    #[arg(short, long)]
    project_name: String,

    /// Task Name
    #[arg(short, long, default_value_t = String::from("Misc"))]
    task_name: String,

    /// Finish Task
    #[arg(short, long)]
    finish: Option<String>,

    /// Resume Task
    #[arg(short, long)]
    resume: Option<String>,
}

fn main() {
    let mut cli = Args::parse();
    
    // Mock project and task data for demonstration
    let mut project = Project {
        name: cli.project_name.clone(),
        tasks: vec![
            Task {
                name: String::from("Task1"),
                sessions: vec![Session {
                    start_time: chrono::Local::now().to_rfc2822().to_string(),
                    end_time: None,
                    state_changes: Some(vec![(Type::Misc, Type::TimePass)]),
                }],
                finished: false
            },
        ],
        tags:vec![],
    };
    
    // Print current status
    print_status(&project);
    
    // Optionally, you can save the updated project data back to your storage system.
    println!("{:#?}", cli);
}
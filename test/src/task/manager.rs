// This file contains the "TaskManager" structure and its methods.
use std::process::Command;

pub struct TaskManager;

impl TaskManager {
    pub fn list_tasks() {
        let output = Command::new("ps")
            .arg("-e")
            .output()
            .expect("Failed to execute command");

        let result = String::from_utf8_lossy(&output.stdout);
        println!("Current Tasks:\n{}", result);
    }
}

use std::process::{Command, Output};

pub fn sudo_exec(cmd:Vec<&str>) -> Output {
    Command::new("sudo")
        .args(cmd.clone())
        .output()
        .expect(format!("failed to execute{:#}",cmd.join(" ")).as_str())
}



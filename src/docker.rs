use std::process::{Command, Stdio};

pub fn docker_exec(args: &[&str]) {
    Command::new("docker")
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .unwrap();
}

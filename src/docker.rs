use std::process::{Command, Stdio};

pub struct Docker;

impl Docker {
    pub fn exec(args: &[&str]) {
        Command::new("docker")
            .args(args)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .unwrap();
    }

    pub fn start_container(language: &str) {
        let args = &[
            "run",
            "--rm",
            &format!("--name=myrias_{}", language),
            "-u1000:1000",
            "-w/tmp/",
            "-dt",
            "--net=none",
            "--cpus=0.25",
            "-m=128m",
            "--memory-swap=128m",
            &format!("myrias_{}:latest", language),
            "/bin/sh",
        ];

        #[cfg(target_os = "linux")]
        args.insert(1, "--runtime=runsc".to_string());

        Docker::exec(args);
    }
}

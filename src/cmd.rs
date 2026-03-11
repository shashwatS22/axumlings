use anyhow::{Context, Result};
use std::path::Path;
use std::process::{Command, Stdio};

pub struct CmdRunner {
    cwd: std::path::PathBuf,
}

impl CmdRunner {
    pub fn build() -> Result<Self> {
        let cwd = std::env::current_dir().context("Failed to get current directory")?;
        Ok(Self { cwd })
    }

    pub fn cwd(&self) -> &Path {
        &self.cwd
    }

    fn cargo(&self, args: &[&str], bin_name: &str, capture_output: bool) -> Command {
        let mut cmd = Command::new("cargo");
        cmd.current_dir(&self.cwd);
        cmd.args(args);
        cmd.arg("-p");
        cmd.arg("exercises");
        cmd.arg("--bin");
        cmd.arg(bin_name);
        cmd.stdin(Stdio::null());
        if capture_output {
            cmd.stdout(Stdio::piped());
            cmd.stderr(Stdio::piped());
        }
        cmd
    }

    pub fn cargo_build<'a>(
        &self,
        bin_name: &str,
        output: Option<&'a mut Vec<u8>>,
    ) -> CmdWithOutput<'a> {
        CmdWithOutput {
            cmd: self.cargo(&["build"], bin_name, output.is_some()),
            output,
        }
    }

    pub fn cargo_test<'a>(
        &self,
        bin_name: &str,
        output: Option<&'a mut Vec<u8>>,
    ) -> CmdWithOutput<'a> {
        let mut cmd = self.cargo(&["test"], bin_name, output.is_some());
        if output.is_some() {
            cmd.arg("--");
            cmd.args(["--color", "always", "--format", "pretty"]);
        }
        CmdWithOutput {
            cmd,
            output,
        }
    }

    pub fn cargo_clippy<'a>(
        &self,
        bin_name: &str,
        output: Option<&'a mut Vec<u8>>,
        extra_args: &[&str],
    ) -> CmdWithOutput<'a> {
        let mut args = vec!["clippy"];
        args.extend_from_slice(extra_args);
        CmdWithOutput {
            cmd: self.cargo(&args, bin_name, output.is_some()),
            output,
        }
    }

    pub fn run_bin(&self, bin_name: &str, mut output: Option<&mut Vec<u8>>) -> Result<bool> {
        let mut child = Command::new("cargo")
            .current_dir(&self.cwd)
            .args(["run", "-p", "exercises", "--bin", bin_name, "--quiet"])
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context("Failed to run exercise binary")?;

        let start = std::time::Instant::now();
        let success = loop {
            match child.try_wait()? {
                Some(status) => break status.success(),
                None => {
                    if start.elapsed() >= std::time::Duration::from_secs(3) {
                        let _ = child.kill();
                        let _ = child.wait();
                        break true;
                    }
                    std::thread::sleep(std::time::Duration::from_millis(100));
                }
            }
        };

        if let Some(ref mut out) = output {
            if let Some(mut stdout) = child.stdout.take() {
                let _ = std::io::copy(&mut stdout, out);
            }
            if let Some(mut stderr) = child.stderr.take() {
                let _ = std::io::copy(&mut stderr, out);
            }
        }

        Ok(success)
    }
}

pub struct CmdWithOutput<'a> {
    cmd: Command,
    output: Option<&'a mut Vec<u8>>,
}

impl CmdWithOutput<'_> {
    pub fn run(&mut self, _label: &str) -> Result<bool> {
        let result = self.cmd.output().context("Failed to run cargo command")?;

        if let Some(ref mut out) = self.output {
            out.extend_from_slice(&result.stdout);
            out.extend_from_slice(&result.stderr);
        }

        Ok(result.status.success())
    }
}

use crate::cmd::CmdRunner;
use anyhow::Result;

pub const OUTPUT_CAPACITY: usize = 1 << 14;

#[derive(Clone)]
pub struct Exercise {
    pub name: String,
    pub dir: Option<String>,
    pub path: String,
    pub test: bool,
    pub strict_clippy: bool,
    pub hint: String,
    pub done: bool,
}

impl Exercise {
    pub fn sol_path(&self) -> String {
        if let Some(ref dir) = self.dir {
            format!("solutions/{}/{}.rs", dir, self.name)
        } else {
            format!("solutions/{}.rs", self.name)
        }
    }

    pub fn run_exercise(
        &self,
        output: Option<&mut Vec<u8>>,
        cmd_runner: &CmdRunner,
    ) -> Result<bool> {
        self.run(&self.name, output, cmd_runner)
    }

    fn run(
        &self,
        bin_name: &str,
        mut output: Option<&mut Vec<u8>>,
        cmd_runner: &CmdRunner,
    ) -> Result<bool> {
        if let Some(ref mut out) = output {
            out.clear();
        }

        let build_success = cmd_runner
            .cargo_build(bin_name, output.as_deref_mut())
            .run("cargo build …")?;
        if !build_success {
            return Ok(false);
        }

        if let Some(ref mut out) = output {
            out.clear();
        }

        if self.test {
            let test_success = cmd_runner
                .cargo_test(bin_name, output.as_deref_mut())
                .run("cargo test …")?;
            if !test_success {
                let _ = cmd_runner.run_bin(bin_name, output.as_deref_mut());
                return Ok(false);
            }
        }

        if let Some(ref mut out) = output {
            out.clear();
        }

        let clippy_args = if self.strict_clippy {
            vec!["--profile", "test", "--", "-D", "warnings"]
        } else {
            vec!["--profile", "test"]
        };
        let clippy_success = cmd_runner
            .cargo_clippy(bin_name, output.as_deref_mut(), &clippy_args)
            .run("cargo clippy …")?;
        let run_success = cmd_runner.run_bin(bin_name, output.as_deref_mut())?;

        Ok(clippy_success && run_success)
    }
}

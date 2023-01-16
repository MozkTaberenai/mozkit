use clap::Parser;
use scriptant::*;

#[derive(clap::Parser)]
struct Args {
    #[command(subcommand)]
    action: Action,
}

#[derive(clap::Subcommand)]
enum Action {
    Fmt,
    Check,
    Test,
}

impl Action {
    fn run(self) -> Result<()> {
        match self {
            Action::Fmt => {
                cmd!("cargo", "fmt").run()?;
                cmd!("taplo", "format", "**/*.toml").run()?;
                Ok(())
            }
            Action::Check => {
                cmd!("cargo", "audit").run()?;
                cmd!("cargo", "check").run()?;
                cmd!("cargo", "clippy").run()?;
                Ok(())
            }
            Action::Test => cmd!(
                "cargo",
                "test",
                "-p",
                "mozkit",
                "--target",
                "wasm32-unknown-unknown"
            )
            .run(),
        }
    }
}

use std::process::ExitCode;

fn main() -> ExitCode {
    if let Err(err) = Args::parse().action.run() {
        echo_err!(err);
        return ExitCode::from(1);
    }
    ExitCode::SUCCESS
}

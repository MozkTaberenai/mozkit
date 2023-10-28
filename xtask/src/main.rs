use scriptant::*;

#[derive(clap::Parser)]
enum Command {
    Fmt,
    Check,
    Test,
}

impl Command {
    fn run(self) -> Result<(), AnyError> {
        match self {
            Command::Fmt => {
                cmd!("cargo", "fmt").run()?;
                cmd!("taplo", "format", "**/*.toml").run()?;
                Ok(())
            }
            Command::Check => {
                cmd!("cargo", "audit").run()?;
                cmd!("cargo", "check").run()?;
                cmd!("cargo", "clippy").run()?;
                Ok(())
            }
            Command::Test => {
                cmd!(
                    "cargo",
                    "test",
                    // "-p",
                    // "mozkit",
                    "--target",
                    "wasm32-unknown-unknown"
                )
                .run()?;
                Ok(())
            }
        }
    }
}

fn main() -> Result<(), AnyError> {
    use clap::Parser;
    Command::parse().run()
}

use std::io::{self, Read};
use std::process::Command;

fn get_history() -> io::Result<()> {
    let history_output = Command::new("history").output()?;

    match history_output {
        Some(output) => expr,
        None => {
            let history_stderr = String::from_utf8(history_output.stderr);
            eprintln!("Failed to retrieve history");
            eprintln!("{}", history_stderr);
        }
    }
    Ok(())
}

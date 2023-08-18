use std::io::{self, Read};
use std::process::Command;
use std::str::from_utf8;
use std::string::FromUtf8Error;

fn get_history() -> Result<String, Box<dyn std::error::Error>> {
    let history_output = Command::new("history").output()?;

    if history_output.status.success() {
        let history_stdout = String::from_utf8(history_output.stdout)?;

        let lines: Vec<&str> = history_stdout.lines().collect();
        if let Some(prev) = lines.last() {
            let output = Command::new("bash").arg("-c").arg(prev).output()?;

            if output.status.success() {
                let stdout = String::from_utf8(output.stdout)?;

                println!("Output of prev cmd: ");
                println!("{}", stdout);
            } else {
                let stderr = String::from_utf8(output.stderr);

                eprintln!("Previous command failed with error:");
                eprintln!("{:?}", stderr);
            }
        } else {
            println!("No previous command found in history");
        }
    } else {
        let history_stderr = String::from_utf8(history_output.stderr);
        eprintln!("Failed to retrieve history");
        eprintln!("{:?}", history_stderr);
    }
    Ok(String::from("some output"))
}

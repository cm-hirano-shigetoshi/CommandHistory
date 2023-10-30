use std::io::{Error, Result};
use std::process::{Command, Stdio};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Please provide one argument");
        return;
    }
    let file_path: &str = &args[1];

    let new_buffer = execute_fzf(file_path);
    let new_cursor = new_buffer.len();
    println!("{} {}", new_cursor, new_buffer);
}

pub fn execute_fzf(path: &str) -> String {
    let fzf_command = get_fzf_command(path);
    let ret = execute_command(fzf_command.as_str());
    return ret.unwrap();
}

pub fn get_fzf_command(path: &str) -> String {
    return format!("cat {} | fzf", path);
}

pub fn execute_command(command: &str) -> Result<String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .stderr(Stdio::inherit())
        .output()?;

    if output.status.success() {
        Ok(String::from_utf8(output.stdout).unwrap())
    } else {
        Err(Error::new(
            std::io::ErrorKind::Other,
            "Command execution failed",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_fzf() {
        let path = "/tmp/powerlog/history";
        let result = execute_fzf(path);
        assert_eq!(result, "/tmp/powerlog/history");
    }

    #[test]
    fn test_execute_command() {
        let result = execute_command("echo aaa").unwrap();
        assert_eq!(result, "aaa\n");
    }
}

use std::io::{Error, Result};
use std::path::Path;
use std::process::{Command, Stdio};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        println!("Please provide one argument");
        return;
    }
    let file_path: &str = &args[1];
    let lbuffer: &str = &args[2];
    let tool_dir: &str = &args[3];
    if Path::new(file_path).exists() {
        let new_buffer = execute_fzf(file_path, lbuffer, tool_dir);
        if new_buffer.len() > 0 {
            let new_cursor = new_buffer.len();
            println!("{} {}", new_cursor, new_buffer);
        }
    }
}

pub fn execute_fzf(path: &str, lbuffer: &str, tool_dir: &str) -> String {
    let fzf_command = get_fzf_command(path, lbuffer, tool_dir);
    return execute_command(fzf_command.as_str()).unwrap_or_else(|_err| String::from(""));
}

pub fn get_fzf_command(path: &str, lbuffer: &str, tool_dir: &str) -> String {
    return format!(
        "tac {} | {}/bash/local.sh | fzf --ansi --query '{}' --bind 'ctrl-r:reload(tac {} | {}/bash/global.sh)'",
        path, tool_dir, lbuffer, path, tool_dir
    );
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
    fn test_execute_command() {
        let result = execute_command("echo aaa").unwrap();
        assert_eq!(result, "aaa\n");
    }
}

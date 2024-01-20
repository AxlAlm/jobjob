use std::{
    io::{Error, ErrorKind},
    process::Output,
};
use tokio::process::Command;

pub fn parse_raw_command(raw_command: &str) -> Result<Command, Error> {
    let raw_command = raw_command.trim();

    if raw_command.is_empty() {
        return Err(Error::new(ErrorKind::Other, "Failed to create command"));
    }

    let parts: Vec<&str> = raw_command.split_whitespace().into_iter().collect();
    let mut command = Command::new(parts[0]);

    for (_i, p) in parts[1..].iter().enumerate() {
        command.arg(p);
    }

    return Ok(command);
}

pub async fn run_command(command: &mut Command) -> Result<Output, Error> {
    let output = command
        .output()
        .await
        .map_err(|err| Error::new(ErrorKind::Other, format!("Command run failed {}", err)))?;
    Ok(output)
}

pub async fn run_raw_command(raw_command: &str) -> Result<Output, Error> {
    let mut command = parse_raw_command(raw_command)?;
    Ok(run_command(&mut command).await?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_raw_command() {
        let test_input = "ls";
        let command = parse_raw_command(test_input)
            .unwrap_or_else(|err| panic!("Failed to create command from valid string: {}", err));

        let mut parts = vec!["ls"];
        let args: Vec<&str> = command
            .as_std()
            .get_args()
            .into_iter()
            .map(|x| x.to_str().unwrap_or_default())
            .collect();
        parts.extend(args);
        let raw_command = parts.join(" ");

        assert_eq!(raw_command, test_input);
    }

    #[test]
    fn test_parse_raw_command_w_args() {
        let test_input = "ls -lgh";
        let command = parse_raw_command(test_input)
            .unwrap_or_else(|err| panic!("Failed to create command from valid string: {}", err));

        let mut parts = vec!["ls"];
        let args: Vec<&str> = command
            .as_std()
            .get_args()
            .into_iter()
            .map(|x| x.to_str().unwrap_or_default())
            .collect();
        parts.extend(args);
        let raw_command = parts.join(" ");
        assert_eq!(raw_command, test_input);
    }

    #[test]
    fn test_parse_raw_command_w_params() {
        let test_input = "ls -lgh .";
        let command = parse_raw_command(test_input)
            .unwrap_or_else(|err| panic!("Failed to create command from valid string: {}", err));

        let mut parts = vec!["ls"];
        let args: Vec<&str> = command
            .as_std()
            .get_args()
            .into_iter()
            .map(|x| x.to_str().unwrap_or_default())
            .collect();
        parts.extend(args);
        let raw_command = parts.join(" ");
        assert_eq!(raw_command, test_input);
    }
}

use crate::Command;
use crate::ParseError;

pub fn parse_command(input: &str) -> Result<Command, ParseError> {
    Ok(Command::Get(String::from("")))
}

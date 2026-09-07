use crate::{Command, ParseError};

pub fn parse_command(input: &str) -> Result<Command, ParseError> {
    let mut fields = input.split_whitespace();

    match fields.next() {
        Some("GET") => {
            let key = fields.next().ok_or(ParseError::MissingArgument)?;

            Ok(Command::Get(key.to_string()))
        }
        Some("SET") => {
            let key = fields.next().ok_or(ParseError::MissingArgument)?;
            let value = fields.next().ok_or(ParseError::MissingArgument)?;

            Ok(Command::Set(key.to_string(), value.to_string()))
        }
        Some("DELETE") => {
            let key = fields.next().ok_or(ParseError::MissingArgument)?;

            Ok(Command::Delete(key.to_string()))
        }
        _ => Err(ParseError::InvalidCommand),
    }
}

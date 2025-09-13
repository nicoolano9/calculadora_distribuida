use crate::server::sv_protocol_errors::ProtocolError;

/// Protocolo del servidor
#[derive(Debug, PartialEq)]
pub enum Command<'a> {
    Op { operator: &'a str, arg: u8 },
    Get,
}

impl<'a> Command<'a> {
    pub fn parse(input: &'a str) -> Result<Command<'a>, ProtocolError> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return Err(ProtocolError::Empty);
        }

        let mut parts = trimmed.split_whitespace();
        let command = parts.next().ok_or(ProtocolError::InvalidCommand)?;

        match command {
            "OP" => {
                // Expect exactly two components: <operator> <operand>
                let operator_string = parts.next().ok_or(ProtocolError::MissingArgument)?;
                let operand_string = parts.next().ok_or(ProtocolError::MissingArgument)?;

                // No extra parts allowed
                if parts.next().is_some() {
                    return Err(ProtocolError::InvalidCommand);
                }

                match operator_string {
                    "+" | "-" | "*" | "/" => {}
                    _ => return Err(ProtocolError::InvalidOperator),
                }

                let operand = match operand_string.parse::<u8>() {
                    Ok(num) => num,
                    Err(_) => return Err(ProtocolError::InvalidArgument),
                };

                Ok(Command::Op {
                    operator: operator_string,
                    arg: operand,
                })
            }

            "GET" => {
                if parts.next().is_some() {
                    return Err(ProtocolError::InvalidCommand);
                }
                Ok(Command::Get)
            }

            // Any other valid command is unexpected
            "OK" | "ERROR" | "VALUE" => Err(ProtocolError::UnexpectedMessage(command.to_string())),

            _ => Err(ProtocolError::InvalidCommand),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_get_ok() {
        assert_eq!(Command::parse("GET"), Ok(Command::Get));
    }

    #[test]
    fn parse_get_with_extra_parts_err() {
        assert_eq!(
            Command::parse("GET extra"),
            Err(ProtocolError::InvalidCommand)
        );
    }

    #[test]
    fn parse_op_ok() {
        assert_eq!(
            Command::parse("OP + 10"),
            Ok(Command::Op {
                operator: "+",
                arg: 10
            })
        );
    }

    #[test]
    fn parse_op_invalid_operator_err() {
        assert_eq!(
            Command::parse("OP ^ 1"),
            Err(ProtocolError::InvalidOperator)
        );
    }

    #[test]
    fn parse_op_invalid_operand_err() {
        assert_eq!(
            Command::parse("OP + asd"),
            Err(ProtocolError::InvalidArgument)
        );
    }

    #[test]
    fn parse_op_missing_arguments_err() {
        assert_eq!(Command::parse("OP"), Err(ProtocolError::MissingArgument));
        assert_eq!(Command::parse("OP +"), Err(ProtocolError::MissingArgument));
    }

    #[test]
    fn parse_op_extra_arguments_err() {
        assert_eq!(
            Command::parse("OP * 2 extra"),
            Err(ProtocolError::InvalidCommand)
        );
    }

    #[test]
    fn parse_empty_err() {
        assert_eq!(Command::parse("   "), Err(ProtocolError::Empty));
        assert_eq!(Command::parse(""), Err(ProtocolError::Empty));
    }

    #[test]
    fn parse_unexpected_message_err() {
        assert_eq!(
            Command::parse("OK"),
            Err(ProtocolError::UnexpectedMessage("OK".to_string()))
        );
        assert_eq!(
            Command::parse("ERROR \"test\""),
            Err(ProtocolError::UnexpectedMessage("ERROR".to_string()))
        );
        assert_eq!(
            Command::parse("VALUE 42"),
            Err(ProtocolError::UnexpectedMessage("VALUE".to_string()))
        );
    }
}

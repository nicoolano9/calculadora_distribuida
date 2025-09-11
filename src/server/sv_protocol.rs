use crate::server::sv_protocol_errors::ProtocolError;

/// Protocolo del servidor
#[derive(Debug, PartialEq)]
pub enum Command<'a> {
    Op {
        operator: &'a str,
        arg: u8,
    },
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

            _ => Err(ProtocolError::InvalidCommand),
        }
    }
}


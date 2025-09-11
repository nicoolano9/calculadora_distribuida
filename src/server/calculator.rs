use crate::server::calculator_errors::CalcError;

#[derive(Default, Debug)]
pub struct Calculator {
    value: u8,
}

impl Calculator {
    pub fn new() -> Self {
        Calculator { value: 0 }
    }

    pub fn value(&self) -> u8 {
        self.value
    }

    pub fn apply(&mut self, operator: &str, operand: u8) -> Result<(), CalcError> {
        match operator {
            "+" => self.value = self.value.wrapping_add(operand),
            "-" => self.value = self.value.wrapping_sub(operand),
            "*" => self.value = self.value.wrapping_mul(operand),
            "/" => self.value = self.value.wrapping_div(operand),
            _ => return Err(CalcError::UnknownOperator),
        }
        Ok(())
    }
}


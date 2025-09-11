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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_is_zero() {
        let c = Calculator::new();
        assert_eq!(c.value(), 0);
    }

    #[test]
    fn addition_updates_value() {
        let mut c = Calculator::default();
        assert!(c.apply("+", 5).is_ok());
        assert_eq!(c.value(), 5);
    }

    #[test]
    fn subtraction_wraps_on_underflow() {
        let mut c = Calculator::new();
        assert!(c.apply("-", 1).is_ok());
        assert_eq!(c.value(), 255u8);
    }

    #[test]
    fn multiplication_wraps_on_overflow() {
        let mut c = Calculator::default();
        c.apply("+", 200).unwrap();
        c.apply("*", 2).unwrap(); // 200 * 2 = 400 -> 400 % 256 = 144
        assert_eq!(c.value(), 144u8);
    }

    #[test]
    fn division_behaves_as_expected() {
        let mut c = Calculator::new();
        c.apply("+", 10).unwrap();
        c.apply("/", 2).unwrap();
        assert_eq!(c.value(), 5u8);
    }

    #[test]
    fn unknown_operator_returns_error() {
        let mut c = Calculator::new();
        let res = c.apply("%", 3);
        assert!(matches!(res, Err(CalcError::UnknownOperator)));
    }
}

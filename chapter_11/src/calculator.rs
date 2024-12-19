// Calculator implementation
pub struct Calculator {
    last_result: f64,
}

impl Calculator {
    pub fn new() -> Calculator {
        Calculator { last_result: 0.0 }
    }

    pub fn add(&mut self, a: f64, b: f64) -> f64 {
        self.last_result = a + b;
        self.last_result
    }

    pub fn subtract(&mut self, a: f64, b: f64) -> f64 {
        self.last_result = a - b;
        self.last_result
    }

    pub fn get_last_result(&self) -> f64 {
        self.last_result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        let mut calc = Calculator::new();
        assert_eq!(calc.add(2.0, 2.0), 4.0);
    }

    #[test]
    fn test_subtraction() {
        let mut calc = Calculator::new();
        assert_eq!(calc.subtract(5.0, 3.0), 2.0);
    }

    #[test]
    fn test_floating_point_precision() {
        let mut calc = Calculator::new();
        let result = calc.add(0.1, 0.2);
        assert!((result - 0.3).abs() < f64::EPSILON);
    }

    #[test]
    fn test_large_numbers() {
        let mut calc = Calculator::new();
        let result = calc.add(1e15, 1e15);
        assert_eq!(result, 2e15);
    }

    #[test]
    fn test_result_history() {
        let mut calc = Calculator::new();
        calc.add(2.0, 3.0);
        assert_eq!(calc.get_last_result(), 5.0);
        calc.subtract(10.0, 3.0);
        assert_eq!(calc.get_last_result(), 7.0);
    }
}

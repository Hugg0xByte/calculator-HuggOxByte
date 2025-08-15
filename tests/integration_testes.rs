use calculator_HuggOxByte::calc1::{add, sub};
use calculator_HuggOxByte::calc2::{multiply, rate};

#[cfg(test)]
mod testes_operacoes_basicas {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_sub() {
        assert_eq!(sub(1, 2), 0);
        assert_eq!(sub(u32::MIN, 1), u32::MIN);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(1, 2), 2);
    }

    #[test]
    fn test_rate() {
        assert_eq!(rate(1, 2), 0);
    }
}

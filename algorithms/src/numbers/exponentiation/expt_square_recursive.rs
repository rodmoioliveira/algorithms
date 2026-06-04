// REFERENCES:
// - Harold Abelson and Gerald Jay Sussman with Julie Sussman. Structure and Interpretation of Computer Programs. Chapter 1: 1.2.4 Exponentiation
//
fn square(i: u32) -> u32 {
    i * i
}

pub fn expt_square_recursive(base: u32, exponent: u32) -> u32 {
    match exponent == 0 {
        true => 1,
        false => match exponent.is_multiple_of(2) {
            true => square(expt_square_recursive(base, exponent / 2)),
            false => base * expt_square_recursive(base, exponent - 1),
        },
    }
}

pub fn example() {
    let res = expt_square_recursive(2, 10);
    eprintln!("{} {:?}", module_path!(), res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let exponents: Vec<u32> = (0..20).collect();
        let results: Vec<u32> = exponents
            .iter()
            .map(|e| expt_square_recursive(2, *e))
            .collect();
        let expected: Vec<u32> = exponents.iter().map(|e| 2_u32.pow(*e)).collect();
        assert_eq!(results, expected);
    }
}

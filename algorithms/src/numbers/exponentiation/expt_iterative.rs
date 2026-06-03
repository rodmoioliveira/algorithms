// REFERENCES:
// - Harold Abelson and Gerald Jay Sussman with Julie Sussman. Structure and Interpretation of Computer Programs. Chapter 1: 1.2.4 Exponentiation
//
pub fn expt_iterative(base: u32, exponent: u32) -> u32 {
    expt_iter(base, exponent, 1)
}

pub fn expt_iter(base: u32, exponent: u32, product: u32) -> u32 {
    match exponent == 0 {
        true => product,
        false => expt_iter(base, exponent - 1, product * base),
    }
}

pub fn example() {
    let res = expt_iterative(2, 10);
    eprintln!("{} {:?}", module_path!(), res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let exponents: Vec<u32> = (0..20).collect();
        let results: Vec<u32> = exponents.iter().map(|e| expt_iterative(2, *e)).collect();
        let expected: Vec<u32> = exponents.iter().map(|e| 2_u32.pow(*e)).collect();
        assert_eq!(results, expected);
    }
}

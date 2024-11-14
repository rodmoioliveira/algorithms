// 5667e8f4e3f572a8f2000039. Mumbling
// https://www.codewars.com/kata/5667e8f4e3f572a8f2000039/train/rust
// This time no story, no theory.
//
// The examples below show you how to write function accum:
//
// Examples:
//
// Accum("abcd") -> "A-Bb-Ccc-Dddd"
// Accum("RqaEzty") -> "R-Qq-Aaa-Eeee-Zzzzz-Tttttt-Yyyyyyy"
// Accum("cwAt") -> "C-Ww-Aaa-Tttt"
//
// The parameter of accum is a string which includes only letters from a..z and A..Z.
//
// ---
pub fn accum(s: &str) -> String {
    s.char_indices()
        .map(|(i, c)| c.to_uppercase().to_string() + &c.to_lowercase().to_string().repeat(i))
        .reduce(|acc, e| acc + "-" + &e)
        .unwrap()
}
// ---

pub fn testcase() {
    let s = "abcd";
    let result = accum(s);
    eprintln!("codewars/5667e8f4e3f572a8f2000039_mumbling: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(
            accum("ZpglnRxqenU"),
            "Z-Pp-Ggg-Llll-Nnnnn-Rrrrrr-Xxxxxxx-Qqqqqqqq-Eeeeeeeee-Nnnnnnnnnn-Uuuuuuuuuuu"
        );
        assert_eq!(
            accum("NyffsGeyylB"),
            "N-Yy-Fff-Ffff-Sssss-Gggggg-Eeeeeee-Yyyyyyyy-Yyyyyyyyy-Llllllllll-Bbbbbbbbbbb"
        );
        assert_eq!(
            accum("MjtkuBovqrU"),
            "M-Jj-Ttt-Kkkk-Uuuuu-Bbbbbb-Ooooooo-Vvvvvvvv-Qqqqqqqqq-Rrrrrrrrrr-Uuuuuuuuuuu"
        );
        assert_eq!(
            accum("EvidjUnokmM"),
            "E-Vv-Iii-Dddd-Jjjjj-Uuuuuu-Nnnnnnn-Oooooooo-Kkkkkkkkk-Mmmmmmmmmm-Mmmmmmmmmmm"
        );
        assert_eq!(
            accum("HbideVbxncC"),
            "H-Bb-Iii-Dddd-Eeeee-Vvvvvv-Bbbbbbb-Xxxxxxxx-Nnnnnnnnn-Cccccccccc-Ccccccccccc"
        );
    }
}

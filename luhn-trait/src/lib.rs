pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

/// Here is the example of how to implement custom Luhn trait
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T: ToString> Luhn for T {
    fn valid_luhn(&self) -> bool {
        let mut codes = self.to_string();
        codes.retain(|c| c.is_digit(10));
        if codes.len() <= 1 {
            return false
        }
    
        self.to_string().chars()
            .filter(|char| !char.is_whitespace())
            .map(|char| char.to_digit(10))
            .rev()
            .enumerate()
            .map(|(pos, optDigit)| if pos % 2 == 1 {optDigit.map(|digit| digit * 2)} else {optDigit})
            .map(|num| num.map(|digit| if digit > 9 {digit - 9} else {digit}))
            .try_fold(0, |sum, digit| digit.map(|d| sum + d)).map_or(false, |res| res % 10 == 0)
    }
}

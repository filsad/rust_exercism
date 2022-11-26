pub struct Luhn {
    code: String
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        if self.code.len() <= 1 {
            return false
        }
    
        self.code.chars()
            .filter(|char| !char.is_whitespace())
            .map(|char| char.to_digit(10))
            .rev()
            .enumerate()
            .map(|(pos, optDigit)| if pos % 2 == 1 {optDigit.map(|digit| digit * 2)} else {optDigit})
            .map(|num| num.map(|digit| if digit > 9 {digit - 9} else {digit}))
            .try_fold(0, |sum, digit| digit.map(|d| sum + d)).map_or(false, |res| res % 10 == 0)
    }
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T: ToString> From<T> for Luhn {
    fn from(input: T) -> Self {
        let mut code = input.to_string();
        
        Luhn { code }
    }
}

use lazy_static::lazy_static;
use regex::Regex;
use regex::RegexSet;

pub struct PasswordReport {
    pub email_captures: Vec<String>,
    pub dates_captures: Vec<String>,
    pub credit_card_numbers_captures: Vec<String>,
    pub entropy: f32,
}
impl PasswordReport {
    pub fn password_analysis(&mut self, password: &str) {
        let mut pool: f32 = 0.0;
        let length: usize = password.len();
        lazy_static! {
            static ref EMAILS: Regex = Regex::new(
                r"(?i)[a-z0-9+[\.]{0,1}?+]{1,32}@([a-z0-9]+([\-]{1}[a-z0-9]+)*\.(com\.[a-z]{2,3}?|com|net\.[a-z]{2,3}?|net|co|co\.[a-z]{2,3}?|gov\.[a-z]{2,3}?|gov|org\.[a-z]{2,3}?|org|edu\.[a-z]{2,3}?|edu|[a-z]{2,3}))",
                )
            .unwrap();

            static ref DATES: Regex = Regex::new(
                r"[0-9]{4}[\.|\-|\\|/]{0,1}?+(((0[13578]|(10|12))[\.|\-|\\|/]{0,1}?+(0[1-9]|[1-2][0-9]|3[0-1]))|(02-(0[1-9]|[1-2][0-9]))|((0[469]|11)[\.|\-|\\|/]{0,1}?+(0[1-9]|[1-2][0-9]|30)))",
                )
            .unwrap();

            static ref CREDIT_CARD_NUMBERS: Regex = Regex::new(
                r"(4[0-9]{12}(?:[0-9]{3})?$)|(^(?:5[1-5][0-9]{2}|222[1-9]|22[3-9][0-9]|2[3-6][0-9]{2}|27[01][0-9]|2720)[0-9]{12}$)|(3[47][0-9]{13})|(^3(?:0[0-5]|[68][0-9])[0-9]{11}$)|(^6(?:011|5[0-9]{2})[0-9]{12}$)|(^(?:2131|1800|35\d{3})\d{11})",
                )
            .unwrap();

            static ref CHAR_MATCHER: RegexSet = RegexSet::new(&[
                r"[[:lower:]]",
                r"[[:upper:]]",
                r"[[:digit:]]",
                r"[[:punct:]]"]).unwrap();

        }
        for capture in EMAILS.captures_iter(password) {
            self.email_captures.push(capture[0].to_string());
        }
        for capture in DATES.captures_iter(password) {
            self.dates_captures.push(capture[0].to_string());
        }
        for capture in CREDIT_CARD_NUMBERS.captures_iter(password) {
            self.credit_card_numbers_captures
                .push(capture[0].to_string());
        }

        let matches: Vec<_> = CHAR_MATCHER.matches(password).into_iter().collect();
        if !matches.is_empty() {
            for c in matches {
                match c {
                    0 => pool += 26.0,
                    1 => pool += 26.0,
                    2 => pool += 10.0,
                    3 => pool += 32.0,
                    _ => pool = 0.0,
                }
            }
            self.entropy = length as f32 * pool.log2();
        }
    }
}

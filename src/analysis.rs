use lazy_static::lazy_static;
use regex::Regex;

pub struct PasswordReport {
    //Sensitive info vectors
    pub email_captures: Vec<String>,
    pub dates_captures: Vec<String>,
    pub credit_card_numbers_captures: Vec<String>,
    //Weak complexity members
}
impl PasswordReport {
    pub fn password_analysis(&mut self, password: &str) {
        /*
        The reason for using to literals to detect TLDs is that iterating over capture groups returns none overlapping captures so  if two emails were next to each other in the password a TLD could eat up part of the next email or worse, it could eat it all up and make it undetectable.
         */
        lazy_static! {
            static ref EMAILS: Regex = Regex::new(
                r"[a-z0-9_+[\.]{0,1}?+]{1,32}@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.(com|net|tn|it|fr|jp|co\.uk|com\.br|de|ru|br|co\.in|it|es|in|ca|ch|com\.au|co\.jp|nl|com\.ar|com\.mx|nl|co\.id|com\.sg|net\.au))",
                )
            .unwrap();
            static ref DATES: Regex = Regex::new(
                r"[0-9]{4}-(((0[13578]|(10|12))-(0[1-9]|[1-2][0-9]|3[0-1]))|(02-(0[1-9]|[1-2][0-9]))|((0[469]|11)-(0[1-9]|[1-2][0-9]|30)))",
                )
            .unwrap();
            static ref CREDIT_CARD_NUMBERS: Regex = Regex::new(
                r"(4[0-9]{12}(?:[0-9]{3})?$)|(^(?:5[1-5][0-9]{2}|222[1-9]|22[3-9][0-9]|2[3-6][0-9]{2}|27[01][0-9]|2720)[0-9]{12}$)|(3[47][0-9]{13})|(^3(?:0[0-5]|[68][0-9])[0-9]{11}$)|(^6(?:011|5[0-9]{2})[0-9]{12}$)|(^(?:2131|1800|35\d{3})\d{11})",
                )
            .unwrap();
        }
        //Maybe implement a workaround to use RegexSet instead of looping over every regex to get the capture.
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
    }
}

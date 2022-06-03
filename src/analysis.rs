use lazy_static::lazy_static;
use regex::RegexSet;
pub fn password_analysis(password: &str) -> Vec<usize> {
    /*
    -Optimize
    -
    */
    lazy_static! {
        static ref SENSITIVE_INFO: RegexSet = RegexSet::new(&[
        //Enhance this regex to match valid dates without punctuation.
        r"[0-9]{4}-(((0[13578]|(10|12))-(0[1-9]|[1-2][0-9]|3[0-1]))|(02-(0[1-9]|[1-2][0-9]))|((0[469]|11)-(0[1-9]|[1-2][0-9]|30)))",
        r"(?i)[a-z]+@[a-z]+\.(com|org|net)",
        r"(4[0-9]{12}(?:[0-9]{3})?$)|(^(?:5[1-5][0-9]{2}|222[1-9]|22[3-9][0-9]|2[3-6][0-9]{2}|27[01][0-9]|2720)[0-9]{12}$)|(3[47][0-9]{13})|(^3(?:0[0-5]|[68][0-9])[0-9]{11}$)|(^6(?:011|5[0-9]{2})[0-9]{12}$)|(^(?:2131|1800|35\d{3})\d{11})",
        ])
        .unwrap();
    }
    let valid_matches: Vec<usize> = SENSITIVE_INFO.matches(password).into_iter().collect();
    //println!("{:?}", valid_matches);

    valid_matches
}

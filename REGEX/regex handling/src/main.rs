use regex::Regex;

fn main() {
    let pattern = r"\d{3}"; 
    let re = Regex::new(pattern).expect("Fail!");

    let text = "The secret code is 404 and 505";

    if re.is_match(text) {
        println!("Found a 3-digit number!");
    }

    for cap in re.find_iter(text) {
        println!("Found: {}", cap.as_str());
    }
}
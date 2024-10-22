use regex::Regex;

fn main() {
    // Create a regex pattern
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();

    // Test if the pattern matches a string
    let date = "2023-10-05";
    if re.is_match(date) {
        println!("The date {} is in the correct format.", date);
    } else {
        println!("The date {} is not in the correct format.", date);
    }
}
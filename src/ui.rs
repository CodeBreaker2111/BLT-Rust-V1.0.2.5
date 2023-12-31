use crate::acceseories::get_input as get_input;

pub fn main() -> (String, String) {
    let mut results = (String::from(""), String::from(""));

    println!("What is the path to your file? : ");
    results.0 = get_input();

    results.1 = format!("{}", results.0.strip_suffix(".blt").unwrap_or("code"));

    return results;
}
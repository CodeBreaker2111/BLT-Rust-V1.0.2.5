use std::io;use std::thread;pub fn main(){println!("Hello world!");thread::sleep_ms(2000);let mut vanimal = String::from("cat");println!("{}", vanimal.as_str());}
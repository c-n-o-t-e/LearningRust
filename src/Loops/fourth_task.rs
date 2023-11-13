use std::env;

fn main() {
    // A palindrome is a word, verse, or sentence that reads the same backward or forward, such as 'Able was I ere I saw Elba,' or a number like 1881.
    // Write a function named is_palindrome() that checks whether a given string is a palindrome or not. 
    // The function should take a string as input and return a boolean value indicating whether the string is a palindrome or not.

    let args: Vec<String> = env::args().collect();
    let string: String = args[1].parse().unwrap();

    println!("Is palindrome {}", is_palindrome(string));
}

fn is_palindrome(s: String) -> bool {
    let s = s.to_lowercase();
    let s = s.chars().filter(|c| c.is_alphanumeric()).collect::<String>();
    let rev = s.chars().rev().collect::<String>();
    s == rev
}
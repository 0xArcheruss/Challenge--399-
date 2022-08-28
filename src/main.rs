use std::fs::read;
use std::io;
use std::str::Chars;

fn main() {
    let mut input_chars = String::new();
    println!("Type a word to get it's numeric value");
    io::stdin()
        .read_line(&mut input_chars)
        .expect("Failed to read Input");
input_chars.pop();
    //println!("{}", input_chars);
    let str_len: usize = input_chars.len();
    let mut chars: Vec<char> = explode_string(&input_chars);
    let mut sum = 0;
    for letter in chars{
        sum = sum + get_char_value(letter);
    }
    let s = input_chars;
    println!("The numeric value of '{}' is {}", s, sum);
}

fn explode_string(string: &String) -> Vec<char> {
    let chars: Vec<char> = string.chars().collect();
    return chars;
}

fn get_char_value(char: char) -> i32 {
    return match char {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        ' ' => 0,
        _ => 0
    }
}
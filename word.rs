// Write a function that takes a string of words separated by spaces and returns the first word it finds in that string.
// If the function doesn’t find a space in the string, the whole string must be one word, so the entire string should be returned.

fn main() {
    let str = String::from("Hello World!");
    let f_word = first_word(&str);
    println!("First word from {str} is {f_word}");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

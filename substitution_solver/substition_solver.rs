use std::collections::HashMap;

fn main() {
    let contents = std::fs::read_to_string("sample.txt").unwrap();
    let mut letter_map: HashMap<char, u8> = HashMap::new();
    for letter in contents.chars() {
        let entry = letter_map.entry(letter).or_insert(0);
        *entry += 1;
    }
    for entry in letter_map {
        println!("{:?}", entry);
    }
}

/*
Improvements:
- Pass filename in
- Help text if filename not passed in
- Errors if file doesn't exist
- Errors if file not ascii
- remove white space and punctuation
- Make all chars upper / lower
 */
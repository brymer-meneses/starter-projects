use std::char;
use std::io;
use std::io::Write;

static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e',
    'f', 'g', 'h', 'i', 'j',
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't',
    'u', 'v', 'w', 'x', 'y',
    'z'
];

pub fn get_input() -> (String, i8) {
    
    print!("Text: ");
    io::stdout().flush().unwrap();

    let mut text = String::new();

    io::stdin()
        .read_line(&mut text)
        .unwrap();

    print!("Key: ");
    io::stdout().flush().unwrap();

    let mut key = String::new();

    io::stdin()
        .read_line(&mut key)
        .unwrap();
    
    let key = key.trim().parse().expect("Error");

    return (text, key)
}   





pub fn decode(text: String, key: i8) {
     // Create a mutable emptry string 
    // that would be used to append 
    // shifted letters later.

    let mut new_text = String::from("");

    // Initialize a variable that 
    // is equal to " ". 
    // This would be used for comparision.
    let space = String::from(" ")
                        .chars()
                        .next()
                        .expect("Error");

    for letter in text.trim().chars() {


        // Check if the letter is a space 
        if letter == space {
            new_text.push(space);

        } else {

            // Get the index of each letter in text
            // to ASCII_LOWER
            let index = ASCII_LOWER
                                .iter()
                                .position(|&r| r == letter)
                                .unwrap();

            // Add the key to the index: perforiming caesar cipher
            let shifted_index = (index as i8 - key) % 26;
            // Convert the shifted_index to a letter
            let shifted_letter = ASCII_LOWER[(shifted_index) as usize];
            // push the letter to new_text
            new_text.push(shifted_letter);
        }

    }
    println!("\nDecoded Text: {} \nWith key: {}", new_text, key);
    println!("=========================")
}

pub fn encode(text: String, key: i8) {

    // Create a mutable emptry string 
    // that would be used to append 
    // shifted letters later.

    let mut new_text = String::from("");


    // Initialize a variable that 
    // is equal to " ". 
    // This would be used for comparision.
    let space = String::from(" ")
                        .chars()
                        .next()
                        .expect("Error");

    for letter in text.trim().chars() {

        // Check if the letter is a space 
        if letter == space {
            new_text.push(space);

        } else {

            // Get the index of each letter in text
            // to ASCII_LOWER
            let index = ASCII_LOWER
                                .iter()
                                .position(|&r| r == letter)
                                .unwrap();

            // Add the key to the index: perforiming caesar cipher
            let shifted_index = (index as i8 + key) % 26;
            // Convert the shifted_index to a letter
            let shifted_letter = ASCII_LOWER[(shifted_index) as usize];
            // push the letter to new_text
            new_text.push(shifted_letter);
        }

    }
    println!("\nEncoded Text: {} \nWith key: {}", new_text, key);
    println!("=========================")
}
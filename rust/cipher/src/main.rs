use std::io::stdin;
use std::io::stdout;
use std::io::Write;

use std::char;

static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e',
    'f', 'g', 'h', 'i', 'j',
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't',
    'u', 'v', 'w', 'x', 'y',
    'z'
];

fn caesar_cipher(decision: u32) {
     // Get text and key
    let mut decision_text = String::new();

    if decision == 1 {
       decision_text = "Decod".to_string();
    } else if decision == 2 {
       decision_text = "Encod".to_string();
    } else {
        println!("Invalid decision!");
    }

    println!("===== {}ing Mode =====", decision_text);
    print!("\nText to be decoded: ");
    stdout().flush().unwrap();
    
    let mut text = String::new();

    stdin()
        .read_line(&mut text)
        .expect("Invalid input");

            
    print!("Enter the key: ");
    stdout().flush().unwrap();

    let mut key = String::new();

    stdin()
        .read_line(&mut key)
        .expect("Invalid Input");
    
    let mut key: i16 = match key.trim().parse() {

        Ok(num) => num,
        Err(_) => return println!("An error has occured"),
    };

    
    if decision == 1 {
        key = - key;
    } 
  

    // Peform Caesar Shift

    //let mut letter_values = vec![];

    let mut new_text = String::from("");
    
    // We need to trim() to delete the whitespaces
    for letter in text.trim().chars() {

        //  Get the position of each element "index" in the 
        // array ASCII_LOWER

        let space = String::from(" ").chars().next().expect("string is empty");

        if letter == space {
            new_text.push(space);
            
            
        } else {
        
        let index = ASCII_LOWER
                        .iter()
                        .position(|&r| r == letter)
                        .unwrap();
            

        // Iterate through letter_values and 
        // perform modulo each element by 26 and add key. 
        let shifted_index = (index as i16 + key) % 26;

        // Convert "shifted_index" to letter
        let shifted_letter = ASCII_LOWER[shifted_index as usize % 26];

        // Push to new_text
        new_text.push(shifted_letter)  
        }
    };


    println!("\n{}ed Text: {}", decision_text, new_text);
}

fn main() {
    // Will include many ciphers
    println!("\nCaesar Cipher");

    loop {
        println!("\nWhat do you want to do?");
        println!("\t - (1) Decode");
        println!("\t - (2) Encode");
        println!("\t - (3) Exit");

        let mut decision = String::new();

        stdin()
            .read_line(&mut decision)
            .expect("Invalid input");

        let decision: u32 = match decision.trim().parse() {
            Ok(num) => num,
            Err(_) => return println!("Invalid input"),
        };

        if decision == 3 {break}

        caesar_cipher(decision);

    }
}

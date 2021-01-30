use std::io;

mod caesar_cipher;


fn main() {
    // Will include many ciphers
    println!("\nCaesar Cipher");

    loop {
        println!("\nWhat do you want to do?");
        println!("\t - (d) Decode");
        println!("\t - (e) Encode");
        println!("\t - (q) Exit");

        let mut decision = String::new();

        io::stdin()
            .read_line(&mut decision)
            .expect("Invalid input");

        let decision = decision.trim();

        if decision == "d".to_string() {
            println!("===== Decoding Mode =====\n");
            let (text, key) = caesar_cipher::get_input();
            
            caesar_cipher::decode(text, key);

        } else if decision == "e".to_string() {
            println!("===== Encoding Mode =====\n");
            let (text, key) = caesar_cipher::get_input();
            
            caesar_cipher::encode(text, key);
        } else if decision == "q".to_string() {
            println!("Quitting... :(((");
            break
        }

    }
}

use std::io;
use std::io::Write;

fn to_celsius(temp: i16) -> f32 {
    let converted_temp: f32 = ((5 as f32)/(9 as f32))*(32 as f32- temp as f32);
    return converted_temp
}

fn to_fahrenheit(temp: i16) -> f32 {
    let converted_temp: f32= (9 as f32/5 as f32)*(temp as f32) + 32 as f32;
    return converted_temp;
}

fn main() {
    println!("Celsius - Fahrenheit Converter");

    loop {
    println!("What temperature unit do you have?");
    println!("\t- (C) Celsius");
    println!("\t- (F) Fahrenheit");

    print!("Temp. Unit: ");
    io::stdout().flush().unwrap();

    let mut input_unit = String::new();

    io::stdin()
        .read_line(&mut input_unit)
        .expect("Invalid input!");
      
    if input_unit.to_lowercase().trim() == "c".to_string() {
        println!("Convert Celsius to Fahrenheit");
        print!("Temperature: "); 
        io::stdout().flush().unwrap();

        let mut temp = String::new();

        io::stdin()
            .read_line(&mut temp)
            .expect("Invalid input!");
        
        let temp: i16 = temp.trim().parse().expect("Invalid input!");
        
        let converted_temp: f32 = to_celsius(temp);

        println!("{} F", converted_temp);

    } else if input_unit.to_lowercase().trim() == "f".to_string() {
        println!("Convert Fahrenheit to Celsius");
        print!("Temperature: "); 
        io::stdout().flush().unwrap();

        let mut temp = String::new();

        io::stdin()
            .read_line(&mut temp)
            .expect("Invalid input!");
        
        let temp: i16 = temp.trim().parse().expect("Invalid input!");
        
        let converted_temp: f32 = to_fahrenheit(temp);

        println!("{} C", converted_temp);
    } else if  input_unit.to_lowercase().trim() == "q".to_string() {
        break
    } else {
        println!("Invalid Choice");
    }
    }

}

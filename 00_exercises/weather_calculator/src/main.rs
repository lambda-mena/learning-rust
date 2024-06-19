use std::io;

fn request_temperature() -> i32 {
    loop {
        println!("Insert the temperature you want to calculate.");
        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line.");

        return user_input
            .trim()
            .parse::<i32>()
            .expect("Please type a valid number.");
    }
}

fn convert_to_fahrenheit() {
    let celsius = request_temperature();
    let fahrenheit = celsius * 9 / 5 + 32;
    println!("The temperature in fahrenheit would be {fahrenheit}");
}

fn convert_to_celsius() {
    let fahrenheit = request_temperature() - 32;
    let celsius = fahrenheit * 5 / 9;
    println!("The temperature in celsius would be {celsius}");
}

fn main() {
    loop {
        println!("Welcome to the weather calculator.");
        println!("Menu");
        println!("(0) Celsius to F. (1) Fahrenheit to C. (Other) Exit.");
        let mut option = String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line.");

        let option: u8 = option.trim().parse().expect("Please type a valid number.");

        match option {
            0 => convert_to_fahrenheit(),
            1 => convert_to_celsius(),
            _ => break,
        };
    }
}

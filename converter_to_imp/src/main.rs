use std::io;

fn main() {
    println!("Hi, you can convert these units: cm, m, km, g, kg, ml, l to imperial.");
     let mut convert = String::new();

     io::stdin()
         .read_line(&mut convert)
         .expect("Failed to read line");

     let input = convert.trim();

    if input.ends_with("ml") {
        // Extract the numeric part of the input
        let number_part = &input[..input.len() - 3]; // Remove the "ml" part
        match number_part.trim().parse::<f64>() {
            Ok(number) => {
                let result = number / 29.574;
                println!("{} ml is {} fl oz", number, result);
            },
            Err(_) => println!("Error: Could not parse the number."),
        }
    }

   if input.ends_with("m") {
        // Extract the numeric part of the input
        let number_part = &input[..input.len() - 3]; // Remove the "m" part
        match number_part.trim().parse::<f64>() {
            Ok(number) => {
                let result = number / 3.281;
                println!("{} m is {} inch", number, result);
            },
            Err(_) => println!("Error: Could not parse the number."),
        }
    }

   if input.ends_with("cm") {
        // Extract the numeric part of the input
        let number_part = &input[..input.len() - 3]; // Remove the "cm" part
        match number_part.trim().parse::<f64>() {
            Ok(number) => {
                let result = number / 2.54;
                println!("{} cm is {} ft", number, result);
            },
            Err(_) => println!("Error: Could not parse the number."),
        }
    }

    if input.ends_with("km") {
        // Extract the numeric part of the input
        let number_part = &input[..input.len() - 3]; // Remove the "km" part
        match number_part.trim().parse::<f64>() {
            Ok(number) => {
                let result = number / 1.609;
                println!("{} km is {} miles", number, result);
            },
            Err(_) => println!("Error: Could not parse the number."),
        }
    }

    if input.ends_with("l") {
        // Extract the numeric part of the input
        let number_part = &input[..input.len() - 3]; // Remove the "l" part
        match number_part.trim().parse::<f64>() {
            Ok(number) => {
                let result = number / 3.785;
                println!("{} l is {} gal", number, result);
            },
            Err(_) => println!("Error: Could not parse the number."),
        }
    }

    if input.ends_with("g") {
        // Extract the numeric part of the input
        let number_part = &input[..input.len() - 3]; // Remove the "g" part
        match number_part.trim().parse::<f64>() {
            Ok(number) => {
                let result = number / 28.35;
                println!("{} g is {} oz", number, result);
            },
            Err(_) => println!("Error: Could not parse the number."),
        }
    }

    if input.ends_with("kg") {
        // Extract the numeric part of the input
        let number_part = &input[..input.len() - 3]; // Remove the "kg" part
        match number_part.trim().parse::<f64>() {
            Ok(number) => {
                let result = number *2.205;
                println!("{} kg is {} lbs", number, result);
            },
            Err(_) => println!("Error: Could not parse the number."),
        }
    }

}

use std::io::{stdin};

fn main() {
    let mut buf;

    let input_fahrenheit = loop {
        buf = String::new();
        println!("Input the unit of temperature you wish to convert");
        stdin().read_line(&mut buf).expect("Could not read input!");
        buf = buf.trim().to_lowercase();
        if buf == "f" || buf == "fahrenheit" {
            break true;
        } else if buf == "c" || buf == "celsius" {
            break false;
        } else {
            println!("Temperature unit could not be parsed, please input the unit");
        };
    };

    let orig_temperature :f64 = loop {
        buf = String::new();
        println!("Input the temperature you wish to convert");
        stdin().read_line(&mut buf).expect("Could not read input!");
        match buf.trim().parse() {
            Ok(num) => break num,
            Err(_) => println!("Please input a number"),
        }
    };

    if input_fahrenheit {
        let new_temperature = (orig_temperature - 32.0) / 1.8;
        println!("The temperature {} degrees Fahrenheit is {} degrees Celsius",
                 orig_temperature, new_temperature);
    } else {
        let new_temperature = orig_temperature * 1.8 + 32.0;
        println!("The temperature {} degrees Fahrenheit is {} degrees Celsius",
                 orig_temperature, new_temperature);
    }
}

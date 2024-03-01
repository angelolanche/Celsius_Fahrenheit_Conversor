use core::f64;
use std::io;

fn main() {
    const CELSIUS: u8 = 0;
    const FIHRENHEIT: u8 = 1;

    let conversion = conversion_type();

    if conversion == CELSIUS {
        let celsius_temp = get_temp(CELSIUS);
        let fihrenheit_temp = convert_to(celsius_temp, conversion);

        println!("your temperature is: {}'F", fihrenheit_temp);
    }

    if conversion == FIHRENHEIT {
        let fihrenheit_temp = get_temp(FIHRENHEIT);
        let celsius_temp = convert_to(fihrenheit_temp, conversion);

        println!("your temperature is: {}'C", celsius_temp);
    }
}

fn conversion_type() -> u8 {
    loop {
        println!("What kind of conversion you want to do?");
        println!("[0]: Celsius -> Fihrenheit");
        println!("[1]: Fihrenheit -> Celsiu");

        let mut option = String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read option line");

        let option: u8 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        return option;
    }
}

fn get_temp(conversion: u8) -> f64 {
    loop {
        if conversion == 0 {
            println!("What is the temperature in Celsius?");
        }

        if conversion == 1 {
            println!("What is the temperature in Fihrenheit?");
        }

        let mut temp = String::new();
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        let temp: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        return temp;
    }
}

fn convert_to(t: f64, conversion: u8) -> f64 {
    if conversion == 0 {
        let temp: f64 = (t * (9.0 / 5.0)) + 32.0;
        temp
    } else {
        let temp: f64 = (t - 32.0) * (5.0 / 9.0);
        temp
    }
}

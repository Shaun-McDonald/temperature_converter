use std::io;

fn scale_converter(x: f64, scale: &str) -> f64 {
    if scale.eq("Celsius"){
        (x * 1.80) + 32.00
    }
    else {
        (x - 32.00) / 1.80
    }
}

fn main() {
    // F = C * 1.8000 + 32.00
    // C = (F - 32) / 1.8
    // use i32
    loop{
        println!("Enter a temperature in Celsius (24 C) or Fahrenheit (-4 F)");

        let mut temperature = String::new();
        io::stdin()
            .read_line(&mut temperature) 
            // read_line returns a Result, which allows to variants Ok and Err
            // .expect handles the Result, here printing a msg if Err occurs
            .expect("Failed to read line");

        // println!("{temperature}");
        let mut iter = temperature.trim().splitn(2, ' ');
        let degrees = match iter.next() {
            Some(val) => val,
            None => {
                println!("Please enter a temperature value!");
                continue;
            }
        };
        let scale = match iter.next() {
            Some(val) => val,
            None => {
                println!("Please enter a scale value! Separate temperature and scale with a space!");
                continue;
            }
        };
        let degrees: f64 = match degrees.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid temperature!");
                continue;
            }
        };
        let scale: char = match scale.trim().parse() {
            Ok(chr) => chr,
            Err(_) => {
                println!("Please enter a single letter (C)elsius or (F)ahrenheit!");
                continue;
            }
        };

        if scale.eq_ignore_ascii_case(&'c') {
            let f_temp = scale_converter(degrees, "Celsius");
            println!("{degrees}° Celsius equals {:.1}° Fahrenheit.", f_temp);
            break;
        } 
        else if scale.eq_ignore_ascii_case(&'f'){
            let f_temp = scale_converter(degrees, "Fahrenheit");
            println!("{degrees}° Fahrenheit equals {:.1}° Celsius.", f_temp);
            break;
        }
        else {
            println!("Please enter a valid scale (C or F)!");
            continue;
        };
    }
}

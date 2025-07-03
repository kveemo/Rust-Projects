use std::io;
use std::cmp::Ordering;

fn main() {

    loop {

        loop{

            println!("Do you want to convert Farenheit to Celsius, or Celsius to Farenheit? 
                (Type 0 for F to C or type 1 for C to F)");
    
            let mut mode = String::new();
    
            io::stdin()
                .read_line(&mut mode)
                .expect("failed to read line");
    
            let mode: u8 = match mode.trim().parse() {
                Ok(num) => {if num == 0 {
                    println!("Mode 0 selected");
                    } else if num == 1 {
                        println!("Mode 1 selected");
                    } else { 
                        println!("Enter a valid mode!");
                    }
                    num},
                Err(_) => {println!("Enter a valid mode!");
                    continue},
            };

        }
    }
}

//Mode 0 = F to C, Mode 1 = C to F
fn convert(mode: u8, value: f32) -> f32 {
    if mode == 0 {
        (value - 30.0)/2.0
    } else {
        (value*2.0)+30.0
    }
} 
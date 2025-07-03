use std::io;

fn main() {
    loop {

        let mut flag = false;

        let mut mode = String::new();
        
        while flag == false{

            println!("Do you want to convert Farenheit to Celsius, or Celsius to Farenheit? 
                (Type 0 for F to C or type 1 for C to F)");
        
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
                    flag = true;
                    num},
                Err(_) => {println!("Enter a valid mode!");
                    continue},
            };

            flag = false;

            while flag == false{

                let mut value = String::new();
                println!("Input the value to convert");
                    
                io::stdin()
                    .read_line(&mut value)
                    .expect("failed to read line");
            
                let value: f32 = match value.trim().parse() {
                    Ok(num) => {flag = true;
                        num},
                    Err(_) => {println!("Enter a number!");
                        continue},
                };

                let converted = convert(mode, value);
                println!("The converted temperature is {converted}");
            } 
        }
    }
}


//Mode 0 = F to C, Mode 1 = C to F
fn convert(mode: u8, value: f32) -> f32 {
    if mode == 0 {
        (value - 32.0)/(9.0/5.0)
    } else {
        (value*(9.0/5.0))+32.0
    }
} 
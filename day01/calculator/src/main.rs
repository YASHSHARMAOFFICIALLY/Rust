use std::io;
fn main(){
    println!("Welcome to Calulator");

    loop {
        let num1 = loop{
            println!("Enter the first number");
            let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
            match input.trim().parse::<i32>(){
                Ok(num)=>break num,
                Err(_)=>{
                    println!("Invalid Number,try again");
                    continue;
                }
            }
        };

        let num2 = loop {
            println!("Enter the second number");
            let mut input = String::new();
             io::stdin().read_line(&mut input).expect("Failed to read line");
             match input.trim().parse::<i32>(){
                Ok(num)=>break num,
                Err(_)=>{
                    println!("Invalid number,try again");
                    continue;
                }
             }
        };

        let operator = loop {
            println!("Enter the operator (+,-,*,/,%");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");

            let op = input.trim();
             if op == "+" || op == "-" || op == "*" || op == "/" || op == "%" {
                break op.to_string();
            } else {
                println!("Invalid operator, try again.");
            }
        };

        let result = match operator.as_str(){
              "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => {
                if num2 == 0 {
                    println!("Cannot divide by zero!");
                    continue;
                }
                num1 / num2
            }
            "%" => {
                if num2 == 0 {
                    println!("Cannot mod by zero!");
                    continue;
                }
                num1 % num2
            }
            _ => unreachable!(),
        };
        println!("Result is {}",result);
        break;
        }
    }

use std::io;
fn main(){
    println!("Welcome to the calculator");

    loop {

        loop{
            println!("Enter the first number");
            let mut num1 = String::new();
                io::stdin()
                .read_line(&mut num1)
                .expect("failed to read input");
            let num1:i32 = match num1.trim().parse() {
                Ok(num)=>break num,
                Err(_)=>{
                    println!("please enter valid number");
                    continue;
                }
            };
        };
        loop {
            println!("Enter the second number");
            let mut num2 = String::new();
            io::stdin()
                .read_line(&mut num2)
                .expect("Failed to read input");

            let num2:i32 = match num2.trim().parse(){
                Ok(num)=>break num,
                Err(_)=>{
                    println!("please enter the valid number");
                    continue;
                }
            };
        };
        loop {
            println!("Enter the operator you want");
            let mut operator = String::new();
            io::stdin()
                .read_line(&mut operator)
                .expect("Failed to read operator");
            let op:String = match operator.trim(){
                Ok(op)=>break op,
                Err(_)=>{
                    println!("please eneter the valid op");
                    continue;
                }
            };
        };
      
    };
}
use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn start(){
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1..101);
    
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
        
        let guess: u8 = match guess.trim().parse() {
            Ok(num)=> num,
            Err(_)=>continue,
        };
        
        println!("You gussed: {}", guess);
    

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            },
        }
    }
}

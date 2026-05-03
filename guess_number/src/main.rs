use std::io;
use rand::Rng;


fn main(){
   let random_number = rand::thread_rng().gen_range(1..=100);
    println!("{random_number}");
    let mut count = 0;
    let mut high = 100;
    let mut low = 0;

   println!("Welcome to Guess the number");
   println!("Pick any number between 1 to 100");

   loop{
    println!("Guess a number (between {} and {}):",low,high);
   

    let mut guess_number = String::new();
        io::stdin()
            .read_line(&mut guess_number)
            .expect("Failed to Read line");
    
    let guess:u32 = match guess_number.trim().parse(){
        Ok(num)=>num,
        Err(_)=> {
            println!("Invalid input,please eneter a valid number");
            continue;
        }
    };

    if guess < low || guess >high {
        println!("Out of range! Try between {} and {}",low,high);
        continue;
    }


    count = count + 1;
    if guess == random_number {
        println!("Correct!");
        println!("You guessed it in {} attempts",count);
        break;
    }
    else if guess < random_number {
        println!("too low");
        low = guess + 1;
    }
    else {
        println!("Too high");
        high = guess -1;
    }
    
    
   }

}
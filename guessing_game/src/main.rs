use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    
    let secret_number = rand::thread_rng().gen_range(1, 101); //[1,100]

    println!("Guess the number!");
    println!("");
    println!("Please input your guess(hint {})", secret_number);

    
    let quit = String::from("quit");

    loop {
        let mut guess = String::new();
        let _io_result = io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        match guess.trim().cmp(&quit) {
            Ordering::Equal => {
                println!("good bye");
                break;
            },
            _ => {},
        };

        println!("Your guessed: [{}]", guess.trim());

        let guess_number: u32 =  guess.trim()
            .parse()
            .expect("number!!!");

       
        
        match guess_number.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}

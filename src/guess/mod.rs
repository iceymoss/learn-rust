
use std::io;
use rand::Rng;
use std::cmp::Ordering;

pub fn guessing_game() {
    println!("Guess the number!");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces:{}", spaces);

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("the secret number is: {}", secret_number);

    loop {
        println!("please input your guess");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("failed to read line");


        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guess: {}", guess);


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        }
    }
}
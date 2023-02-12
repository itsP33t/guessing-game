use rand::Rng;
// use std::io::stdin;
use colored::Colorize;
use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut rng = rand::thread_rng();
    'outer: loop {
    // let number: i32 = 10;
    let number: i32 = rng.gen_range(0..100);
    
    println!("{}", "Guess a number".blue());
    println!("{}", "The number is between 0 - 100".bold());

    loop {
        let mut line = String::new();

        let input = stdin().read_line(&mut line);
        let guess: Option<i32> = input.ok().map_or(None, |_| line.trim().parse().ok());

        match guess {
            None => println!("{}", "Guess a number".blue()),
            Some(n) if n == number => {
                println!("{}", "You guessed the number, congratulations!".color("green").bold());
                pause();
                break 'outer;
            }
            Some(n) if n < number => println!("{}", "Too low".color("red").bold()),
            Some(n) if n > number => println!("{}", "Too High".color("red").bold()),
            Some(_) => println!("Error")
            
        }
    }
}
}

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

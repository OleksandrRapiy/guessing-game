use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn enter_number() -> i32 {
    let mut number = String::new();

    println!("Guess the numbers: ");

    io::stdin().read_line(&mut number).expect("Failed to read line");

    return number.trim().parse().expect("Please type a number!");
}

fn number_generator(min: i32, max: i32) -> i32 {
    return rand::thread_rng().gen_range(min..=max);
}

fn main() {
    let random_number = number_generator(1, 10);

    loop {
        let number = enter_number();

        match number.cmp(&random_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }

        println!("Play again Y/n");
        let mut choise = String::new();
        io::stdin().read_line(&mut choise).expect("Failed to read line");

        if choise.trim() == "n" {
            break;
        }
    }
}
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=101);

    loop{
        println!("Please input your guess.");

        let mut guess = String::new();

        std::io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse()
        {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number)
        {
            std::cmp::Ordering::Less => println!("Higher"),
            std::cmp::Ordering::Greater => println!("Lower"),
            std::cmp::Ordering::Equal =>{
                println!("You guessed it right!");
                break;
            }
        }
    }
}

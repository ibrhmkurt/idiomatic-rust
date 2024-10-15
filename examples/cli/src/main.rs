use idiomatic_rust::*;

fn main() {
    println!("Parsing Currency");
    println!("Enter Quantity and Currency Unit.");

    loop {
        println!("Please input your Currency.");

        let mut currency = String::new();

        std::io::stdin()
            .read_line(&mut currency)
            .expect("Failed to read line");

        match currency.trim().parse::<Money>() {
            Ok(m) => {
                println!("{m}")
            },
            Err(e) => {
                println!("{e}")
            }
        };

        println!("You guessed: {currency}");
    }
}
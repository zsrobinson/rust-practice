use std::io::{self, Write};

fn main() {
    print!("How many fizzes to buzz? ");
    let _ = io::stdout().flush();

    let mut count = String::new();
    io::stdin()
        .read_line(&mut count)
        .expect("Failed to read line");

    let count: u32 = count.trim().parse().expect("Make sure to type a number!");

    for i in 1..=count {
        let mut output = String::new();

        if i % 3 == 0 {
            output += "fizz"
        };

        if i % 5 == 0 {
            output += "buzz";
        }

        if output == "" {
            output = i.to_string();
        }

        println!("{output}");
    }
}

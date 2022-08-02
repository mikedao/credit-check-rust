use std::io::stdin;

fn main() {
    println!("This is Credit Check!");
    println!("Please enter the credit card number:");

    let number = what_is_your_number();

    println!("You have entered: {}", number);

    let mut split: Vec<_> = number.split("").collect();

    split.remove(0);
    split.pop();

    println!("{:?}", split);
}


fn what_is_your_number() -> String {
    let mut number = String::new();
    stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    number
        .trim()
        .to_lowercase()
}
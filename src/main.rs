use std::io::stdin;

fn main() {
    let num1 = read_user_input_int();
    let num2 = read_user_input_int();

    println!("{} + {} = {}", num1, num2, num1 + num2);
}

fn read_user_input_int() -> u32 {
    let mut input = String::new();
    println!("Please enter a number:");
    stdin()
        .read_line(&mut input)
        .expect("Sorry, I couldn't read that.");
    let input_int = match input.trim().parse::<u32>() {
        Ok(i) => i,
        Err(..) => 0,
    };
    return input_int;
}

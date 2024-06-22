use std::io;

fn read_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn parse_number(input: &str) -> f64 {
    input.parse::<f64>().expect("Please enter a valid number")
}

fn worker() {
    let num_1_string = read_input("Enter the first number: ");
    let num_1 = parse_number(&num_1_string);

    let operator = read_input("Enter an operator (+, -, /, *): ");

    let num_2_string = read_input("Enter the second number: ");
    let num_2 = parse_number(&num_2_string);

    let result = match operator.as_str() {
        "+" => num_1 + num_2,
        "-" => num_1 - num_2,
        "*" => num_1 * num_2,
        "/" => {
            if num_2 != 0.0 {
                num_1 / num_2
            } else {
                println!("Error: Division by zero is not allowed");
                return;
            }
        }

        _ => {
            println!("Error: invalid operator");
            return;
        }
    };
    println!(
        "The result of {} {} {} is :{}",
        num_1, operator, num_2, result
    )
}
fn main() {
    worker()
}

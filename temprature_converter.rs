use std::io;

fn main() {
    let mut temperature_str = String::new();
    let mut unit_str = String::new();

    print!("Enter temperature value: ");
    io::stdout().flush().unwrap(); // Ensure prompt is displayed before input
    io::stdin().read_line(&mut temperature_str).unwrap();
    let temperature: f64 = temperature_str.trim().parse().unwrap();

    print!("Enter unit (C for Celsius, F for Fahrenheit): ");
    io::stdout().flush().unwrap(); // Ensure prompt is displayed before input
    io::stdin().read_line(&mut unit_str).unwrap();
    let unit = unit_str.trim().chars().next().unwrap().to_uppercase().next().unwrap();

    let converted_temp = match unit {
        'C' => c_to_f(temperature),
        'F' => f_to_c(temperature),
        _ => panic!("Invalid unit. Please enter 'C' or 'F'."),
    };

    let new_unit = if unit == 'C' { "F" } else { "C" };
    println!("Converted temperature: {:.2}°{}", converted_temp, new_unit);
}

fn c_to_f(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn f_to_c(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

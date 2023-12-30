use std::io;

enum NumericBase {
    Binary,
    Octal,
    Decimal,
    Hexadecimal,
}

impl NumericBase {
    fn to_string(&self, value: u32) -> String {
        match self {
            NumericBase::Binary => format!("{:b}", value),
            NumericBase::Octal => format!("{:o}", value),
            NumericBase::Decimal => value.to_string(),
            NumericBase::Hexadecimal => format!("{:X}", value),
        }
    }

    fn from_str(input: &str) -> Option<NumericBase> {
        match input.trim() {
            "2" => Some(NumericBase::Binary),
            "8" => Some(NumericBase::Octal),
            "10" => Some(NumericBase::Decimal),
            "16" => Some(NumericBase::Hexadecimal),
            _ => None,
        }
    }
}

fn convert(base_from: NumericBase, base_to: NumericBase, value: &str) -> Result<String, &'static str> {
    let parsed_value = match base_from {
        NumericBase::Binary => u32::from_str_radix(value, 2),
        NumericBase::Octal => u32::from_str_radix(value, 8),
        NumericBase::Decimal => value.parse::<u32>(),
        NumericBase::Hexadecimal => u32::from_str_radix(value, 16),
    };

    parsed_value.map_or(Err("Invalid input"), |val| Ok(base_to.to_string(val)))
}

fn get_user_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn main() {
    println!("Welcome to Numeric System Converter");
    println!("Available Bases:");
    println!("2. Binary");
    println!("8. Octal");
    println!("10. Decimal");
    println!("16. Hexadecimal");

    let base_from = loop {
        if let Some(base) = NumericBase::from_str(&get_user_input("Enter the base to convert FROM (2, 8, 10, 16): ")) {
            break base;
        } else {
            println!("Invalid base. Please enter a valid base number.");
        }
    };

    let base_to = loop {
        if let Some(base) = NumericBase::from_str(&get_user_input("Enter the base to convert TO (2, 8, 10, 16): ")) {
            break base;
        } else {
            println!("Invalid base. Please enter a valid base number.");
        }
    };

    let value_to_convert = get_user_input("Enter the value to convert: ");

    match convert(base_from, base_to, &value_to_convert) {
        Ok(result) => println!("Conversion result: {}", result),
        Err(err) => println!("Conversion error: {}", err),
    }
}

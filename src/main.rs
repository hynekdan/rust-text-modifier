use convert_case::{Case, Casing};
use slug::slugify;

use std::env;
use std::io::{self, Write};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

#[derive(EnumIter)]
enum Operation {
    CamelCase,
    LowerCase,
    NoSpaces,
    Slugify,
    SnakeCase,
    UpperCase,
}

impl Operation {
    fn from_str(s: &str) -> Option<Operation> {
        match s.to_lowercase().as_str() {
            "camelcase" => Some(Operation::CamelCase),
            "lowercase" => Some(Operation::LowerCase),
            "no-spaces" => Some(Operation::NoSpaces),
            "slugify" => Some(Operation::Slugify),
            "snakecase" => Some(Operation::SnakeCase),
            "uppercase" => Some(Operation::UpperCase),
            _ => None,
        }
    }

    fn to_str(self) -> String {
        match self {
            Operation::CamelCase => String::from("camelcase"),
            Operation::LowerCase => String::from("lowercase"),
            Operation::NoSpaces => String::from("no-spaces"),
            Operation::Slugify => String::from("slugify"),
            Operation::SnakeCase => String::from("snakecase"),
            Operation::UpperCase => String::from("uppercase"),
        }
    }

    fn print_available_operations(){
        println!("Available operations are: ");
        for enum_variant in Operation::iter(){
            println!("{}", enum_variant.to_str());
        }
    }

    fn perform_operation(self, s: &str) -> String {
        match self {
            Self::CamelCase => s.to_case(Case::Camel),
            Self::LowerCase => s.to_lowercase(),
            Self::NoSpaces => s.replace(" ", ""),
            Self::Slugify => slugify(s),
            Self::SnakeCase => s.to_case(Case::Snake),
            Self::UpperCase => s.to_uppercase(),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", args[0]);
    if args.len() < 2 {
        println!("It's required to pass an argument specifying the operation.");
        Operation::print_available_operations();

    } else {
        println!("{}", args[1]);
        let operation = Operation::from_str(&args[1]);
        match operation {
            Some(op) => {
                let input = get_input("Insert string to modify: ");
                println!("{} -> {}", input, op.perform_operation(&input));
            }
            None => {
                println!("Incorrect operation {}", args[1]);
                Operation::print_available_operations();
            },
        }
    }
}

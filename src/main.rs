use convert_case::{Case, Casing};
use slug::slugify;

use std::env;
use std::error::Error;
use std::fmt;
use std::io::{self, Write};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use csv::{ReaderBuilder, StringRecord};

use prettytable::{Cell, Row, Table};

#[derive(Debug)]
struct OperationError(String);

impl fmt::Display for OperationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Operation error: {}", self.0)
    }
}

impl Error for OperationError {}

// Struct for CSV handling with Display trait
struct CsvTable {
    headers: StringRecord,
    records: Vec<StringRecord>,
}

impl fmt::Display for CsvTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut table = Table::new();

        let mut header_cells = Vec::new();
        for header in &self.headers {
            header_cells.push(Cell::new(header).style_spec("bcB"));
        }
        table.add_row(Row::new(header_cells));

        for record in &self.records {
            let mut row_cells = Vec::new();
            for field in record {
                row_cells.push(Cell::new(field));
            }
            table.add_row(Row::new(row_cells));
        }

        write!(f, "{}", table)
    }
}

fn get_input(prompt: &str) -> Result<String, Box<dyn Error>> {
    println!("{}", prompt);
    if prompt.contains("CSV") {
        println!("Enter your CSV data (enter an empty line to finish):");
        let mut input = String::new();
        loop {
            let mut line = String::new();
            io::stdin().read_line(&mut line)?;
            if line.trim().is_empty() {
                break;
            }
            input.push_str(&line);
        }
        Ok(input)
    } else {
        print!("> ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input.trim().to_string())
    }
}

#[derive(EnumIter)]
enum Operation {
    CamelCase,
    Csv,
    LowerCase,
    NoSpaces,
    Slugify,
    SnakeCase,
    UpperCase,
}

impl Operation {
    fn from_str(s: &str) -> Result<Operation, Box<dyn Error>> {
        match s.to_lowercase().as_str() {
            "camelcase" => Ok(Operation::CamelCase),
            "csv" => Ok(Operation::Csv),
            "lowercase" => Ok(Operation::LowerCase),
            "no-spaces" => Ok(Operation::NoSpaces),
            "slugify" => Ok(Operation::Slugify),
            "snakecase" => Ok(Operation::SnakeCase),
            "uppercase" => Ok(Operation::UpperCase),
            _ => Err(Box::new(OperationError(format!(
                "Invalid operation: {}",
                s
            )))),
        }
    }

    fn to_str(&self) -> String {
        match self {
            Operation::CamelCase => String::from("camelcase"),
            Operation::Csv => String::from("csv"),
            Operation::LowerCase => String::from("lowercase"),
            Operation::NoSpaces => String::from("no-spaces"),
            Operation::Slugify => String::from("slugify"),
            Operation::SnakeCase => String::from("snakecase"),
            Operation::UpperCase => String::from("uppercase"),
        }
    }

    fn print_available_operations() {
        eprintln!("Available operations are:");
        for operation in Operation::iter() {
            eprintln!("  {}", operation.to_str());
        }
    }
}

fn process_camel_case(input: &str) -> Result<String, Box<dyn Error>> {
    Ok(input.to_case(Case::Camel))
}

fn process_csv(input: &str) -> Result<String, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new()
        .trim(csv::Trim::All)
        .flexible(true)
        .from_reader(input.as_bytes());

    let headers = reader.headers()?.clone();
    if headers.is_empty() {
        return Err(Box::new(OperationError("CSV has no headers".to_string())));
    }

    let records: Result<Vec<StringRecord>, _> = reader.records().collect();
    let records = records?;
    if records.is_empty() {
        return Err(Box::new(OperationError("CSV has no data rows".to_string())));
    }

    let csv_table = CsvTable { headers, records };
    Ok(format!("{}", csv_table))
}

fn process_lower_case(input: &str) -> Result<String, Box<dyn Error>> {
    Ok(input.to_lowercase())
}

fn process_no_spaces(input: &str) -> Result<String, Box<dyn Error>> {
    Ok(input.replace(" ", ""))
}

fn process_slugify(input: &str) -> Result<String, Box<dyn Error>> {
    Ok(slugify(input))
}

fn process_snake_case(input: &str) -> Result<String, Box<dyn Error>> {
    Ok(input.to_case(Case::Snake))
}

fn process_upper_case(input: &str) -> Result<String, Box<dyn Error>> {
    Ok(input.to_uppercase())
}

fn process_operation(op: Operation, input: &str) -> Result<String, Box<dyn Error>> {
    match op {
        Operation::CamelCase => process_camel_case(input),
        Operation::LowerCase => process_lower_case(input),
        Operation::NoSpaces => process_no_spaces(input),
        Operation::Slugify => process_slugify(input),
        Operation::SnakeCase => process_snake_case(input),
        Operation::UpperCase => process_upper_case(input),
        Operation::Csv => process_csv(input),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Error: It's required to pass an argument specifying the operation.");
        Operation::print_available_operations();
        return Ok(());
    }

    let operation = match Operation::from_str(&args[1]) {
        Ok(op) => op,
        Err(e) => {
            eprintln!("Error: {}", e);
            Operation::print_available_operations();
            return Ok(());
        }
    };

    println!("Selected operation: {}", operation.to_str());

    let input = if matches!(operation, Operation::Csv) {
        get_input("Please enter your CSV data:")?
    } else {
        get_input("Insert string to modify:")?
    };

    match process_operation(operation, &input) {
        Ok(result) => println!("{}", result),
        Err(e) => eprintln!("Error processing input: {}", e),
    }

    Ok(())
}

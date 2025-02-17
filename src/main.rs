use convert_case::{Case, Casing};
use slug::slugify;

use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io;
use std::thread;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use csv::{ReaderBuilder, StringRecord};

use prettytable::{Cell, Row, Table};

use flume::{Receiver, Sender};

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

#[derive(Debug, EnumIter)]
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

fn process_csv(file_path: &str) -> Result<String, Box<dyn Error>> {
    let file = File::open(file_path)
        .map_err(|e| OperationError(format!("Failed to open file '{}': {}", file_path, e)))?;

    let mut reader = ReaderBuilder::new()
        .trim(csv::Trim::All)
        .flexible(true)
        .from_reader(file);

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

#[derive(Debug)]
struct Command {
    operation: Operation,
    input: String,
}

fn input_thread(tx: Sender<Command>) -> Result<(), Box<dyn Error>> {
    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input)? == 0 {
            break;
        }

        // Split input into operation and data
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        if parts.len() < 2 {
            eprintln!("Error: Expected format: <operation> <input>");
            Operation::print_available_operations();
            continue;
        }

        match Operation::from_str(parts[0].trim()) {
            Ok(operation) => {
                // Consider everything after first space to be input data
                let input = parts[1..].join(" ");
                if let Err(e) = tx.send(Command { operation, input }) {
                    eprintln!("Error sending command: {}", e);
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                Operation::print_available_operations();
            }
        }
    }
    Ok(())
}

fn processing_thread(rx: Receiver<Command>) -> Result<(), Box<dyn Error>> {
    while let Ok(command) = rx.recv() {
        println!("Selected operation: {}", command.operation.to_str());

        match process_operation(command.operation, &command.input) {
            Ok(result) => println!("{}", result),
            Err(e) => eprintln!("Error processing input: {}", e),
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let (tx, rx) = flume::unbounded();

    let input_handle = thread::spawn(move || {
        if let Err(e) = input_thread(tx) {
            eprintln!("Input thread error: {}", e);
        }
    });

    let processing_handle = thread::spawn(move || {
        if let Err(e) = processing_thread(rx) {
            eprintln!("Processing thread error: {}", e);
        }
    });

    if let Err(e) = input_handle.join() {
        return Err(Box::new(OperationError(format!(
            "Input thread panicked: {:?}",
            e
        ))));
    }

    if let Err(e) = processing_handle.join() {
        return Err(Box::new(OperationError(format!(
            "Processing thread panicked: {:?}",
            e
        ))));
    }

    Ok(())
}

// TODO add some unit tests and integration tests
// TODO divide code into modules
// TODO update README
// TODO implement properly traits (FromStr)
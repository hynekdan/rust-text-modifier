# String Transformer

A command-line utility written in Rust for performing various string transformations.

## Features

- Multiple string transformation operations:
  - CamelCase: Converts text to camelCase format
  - LowerCase: Converts text to lowercase
  - NoSpaces: Removes all spaces from text
  - Slugify: Creates URL-friendly slugs
  - SnakeCase: Converts text to snake_case format
  - UpperCase: Converts text to UPPERCASE

## Installation

1. Ensure you have Rust installed on your system
2. Clone this repository
3. Build the project:
```bash
cargo build --release
```

## Dependencies

- convert_case: For case conversion operations
- slug: For URL-friendly slug generation
- strum: For enum iteration
- strum_macros: For enum iteration macros

## Usage

Run the program with one of the available operations as an argument:

```bash
./string_transformer <operation>
```

Example:
```bash
./string_transformer camelcase
Insert string to modify: hello world
hello world -> helloWorld
```
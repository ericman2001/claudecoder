use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

// Import the tests module (only included in test builds)
mod tests;

#[derive(Debug, PartialEq)]
pub enum Operation {
    Mean,
    Sum,
}

/// Main entry point for the arithmetic calculator.
/// 
/// Expects an operation (mean/sum) and a file path as command-line arguments.
/// Reads the file and calculates either the mean or sum of all valid numeric values.
fn main() {
    // Collect command-line arguments into a vector
    let args: Vec<String> = env::args().collect();
    
    // Check if both operation and file path were provided
    if args.len() < 3 {
        println!("Usage: {} <operation> <file_path>", args[0]);
        println!("Operations: mean, sum");
        return;
    }
    
    // Parse the operation argument
    let operation = match args[1].to_lowercase().as_str() {
        "mean" => Operation::Mean,
        "sum" => Operation::Sum,
        _ => {
            println!("Invalid operation. Use 'mean' or 'sum'");
            return;
        }
    };
    
    // Extract the file path from the arguments
    let file_path = &args[2];
    
    // Calculate the result and handle any errors
    match calculate_from_file(file_path, operation) {
        Ok(result) => println!("{}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
}

/// Calculates either the arithmetic mean or sum of numeric values in a file.
/// 
/// # Arguments
/// * `file_path` - Path to the file containing numeric values (one per line)
/// * `operation` - The operation to perform (Mean or Sum)
/// 
/// # Returns
/// * `Ok(f64)` - The calculated result (mean or sum)
/// * `Err(io::Error)` - If the file cannot be opened or read
/// 
/// # Behavior
/// - Ignores blank lines
/// - Ignores lines that cannot be parsed as f64
/// - For mean: Returns 0.0 if no valid numbers are found (avoids divide by zero)
/// - For sum: Returns 0.0 if no valid numbers are found
pub fn calculate_from_file(file_path: &str, operation: Operation) -> io::Result<f64> {
    // Open the file and create a buffered reader for efficient line-by-line reading
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    
    // Initialize counters for sum and count of valid numbers
    let mut sum = 0.0;
    let mut count = 0;
    
    // Process each line in the file
    for line in reader.lines() {
        // Handle potential I/O errors when reading lines
        let line = line?;
        // Remove leading/trailing whitespace
        let trimmed = line.trim();
        
        // Skip empty lines
        if trimmed.is_empty() {
            continue;
        }
        
        // Try to parse the line as a floating-point number
        // If successful, add it to our sum and increment count
        if let Ok(num) = trimmed.parse::<f64>() {
            sum += num;
            count += 1;
        }
        // Non-numeric lines are silently ignored
    }
    
    // Calculate result based on operation
    match operation {
        Operation::Mean => {
            // Handle the case where no valid numbers were found
            // This prevents divide by zero errors
            if count == 0 {
                Ok(0.0)
            } else {
                Ok(sum / count as f64)
            }
        }
        Operation::Sum => Ok(sum),
    }
}

/// Legacy function for backward compatibility in tests
pub fn calculate_mean_from_file(file_path: &str) -> io::Result<f64> {
    calculate_from_file(file_path, Operation::Mean)
}
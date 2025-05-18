use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

// Import the tests module (only included in test builds)
mod tests;

/// Main entry point for the arithmetic mean calculator.
/// 
/// Expects a file path as the first command-line argument.
/// Reads the file and calculates the mean of all valid numeric values.
fn main() {
    // Collect command-line arguments into a vector
    let args: Vec<String> = env::args().collect();
    
    // Check if a file path was provided (args[0] is the program name)
    if args.len() < 2 {
        println!("Please provide a file path as a parameter");
        return;
    }
    
    // Extract the file path from the arguments
    let file_path = &args[1];
    
    // Calculate the mean and handle any errors
    match calculate_mean_from_file(file_path) {
        Ok(mean) => println!("{}", mean),
        Err(e) => eprintln!("Error: {}", e),
    }
}

/// Calculates the arithmetic mean of numeric values in a file.
/// 
/// # Arguments
/// * `file_path` - Path to the file containing numeric values (one per line)
/// 
/// # Returns
/// * `Ok(f64)` - The arithmetic mean of all valid numbers
/// * `Err(io::Error)` - If the file cannot be opened or read
/// 
/// # Behavior
/// - Ignores blank lines
/// - Ignores lines that cannot be parsed as f64
/// - Returns 0.0 if no valid numbers are found (avoids divide by zero)
pub fn calculate_mean_from_file(file_path: &str) -> io::Result<f64> {
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
    
    // Handle the case where no valid numbers were found
    // This prevents divide by zero errors
    if count == 0 {
        return Ok(0.0);
    }
    
    // Calculate and return the arithmetic mean
    Ok(sum / count as f64)
}
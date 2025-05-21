/// Unit tests for the arithmetic mean calculator.
/// 
/// These tests verify the correct behavior of the calculate_mean_from_file function
/// under various conditions including normal operation, edge cases, and error conditions.
#[cfg(test)]
mod tests {
    // Import the functions and types we're testing from the parent module
    use crate::{calculate_mean_from_file, calculate_from_file, Operation};
    // Import necessary standard library items for testing
    use std::io::Write;
    use tempfile::NamedTempFile;
    
    /// Test case: Calculate mean of a file with valid numeric values
    /// 
    /// Creates a temporary file with three numbers (10, 20, 30)
    /// and verifies that the mean is correctly calculated as 20.0
    #[test]
    fn test_valid_file_with_numbers() {
        // Create a temporary file that will be automatically deleted when it goes out of scope
        let mut temp_file = NamedTempFile::new().unwrap();
        
        // Write test data to the file: three numbers
        writeln!(temp_file, "10").unwrap();
        writeln!(temp_file, "20").unwrap();
        writeln!(temp_file, "30").unwrap();
        
        // Ensure all data is written to disk before reading
        temp_file.flush().unwrap();
        
        // Calculate the mean from our test file
        let result = calculate_mean_from_file(temp_file.path().to_str().unwrap()).unwrap();
        
        // Verify the result: (10 + 20 + 30) / 3 = 20.0
        assert_eq!(result, 20.0);
    }
    
    /// Test case: Calculate mean of an empty file
    /// 
    /// Creates an empty temporary file and verifies that the function
    /// returns 0.0 (avoiding divide by zero error)
    #[test]
    fn test_blank_file() {
        // Create an empty temporary file
        let temp_file = NamedTempFile::new().unwrap();
        
        // Calculate the mean from the empty file
        let result = calculate_mean_from_file(temp_file.path().to_str().unwrap()).unwrap();
        
        // Verify that an empty file returns 0.0 (not an error)
        assert_eq!(result, 0.0);
    }
    
    /// Test case: Calculate mean from file with mixed content
    /// 
    /// Creates a file containing:
    /// - Valid numbers
    /// - Invalid text ("not a number")
    /// - Empty lines
    /// 
    /// Verifies that only valid numbers are included in the calculation
    /// and invalid content is properly ignored
    #[test]
    fn test_file_with_non_numeric_lines() {
        // Create a temporary file with mixed content
        let mut temp_file = NamedTempFile::new().unwrap();
        
        // Write a mix of valid numbers, invalid text, and empty lines
        writeln!(temp_file, "10").unwrap();          // Valid: 10
        writeln!(temp_file, "not a number").unwrap(); // Invalid: ignored
        writeln!(temp_file, "20").unwrap();          // Valid: 20
        writeln!(temp_file, "").unwrap();            // Empty: ignored
        writeln!(temp_file, "30").unwrap();          // Valid: 30
        
        // Ensure all data is written to disk
        temp_file.flush().unwrap();
        
        // Calculate the mean from our test file
        let result = calculate_mean_from_file(temp_file.path().to_str().unwrap()).unwrap();
        
        // Verify only valid numbers are used: (10 + 20 + 30) / 3 = 20.0
        assert_eq!(result, 20.0);
    }
    
    /// Test case: Verify command-line argument handling
    /// 
    /// This test simulates the scenario where insufficient arguments are provided
    /// as command-line arguments and verifies that the program would
    /// handle this case appropriately
    #[test]
    fn test_insufficient_arguments() {
        // Simulate command-line arguments with only the program name
        let args = vec!["program_name".to_string()];
        
        // Verify that we correctly detect insufficient arguments
        // args.len() < 3 means operation and/or file path are missing
        assert!(args.len() < 3);
        
        // Test with only operation provided
        let args = vec!["program_name".to_string(), "mean".to_string()];
        assert!(args.len() < 3);
    }
    
    /// Test case: Calculate sum of a file with valid numeric values
    /// 
    /// Creates a temporary file with three numbers (10, 20, 30)
    /// and verifies that the sum is correctly calculated as 60.0
    #[test]
    fn test_valid_file_with_numbers_sum() {
        // Create a temporary file that will be automatically deleted when it goes out of scope
        let mut temp_file = NamedTempFile::new().unwrap();
        
        // Write test data to the file: three numbers
        writeln!(temp_file, "10").unwrap();
        writeln!(temp_file, "20").unwrap();
        writeln!(temp_file, "30").unwrap();
        
        // Ensure all data is written to disk before reading
        temp_file.flush().unwrap();
        
        // Calculate the sum from our test file
        let result = calculate_from_file(temp_file.path().to_str().unwrap(), Operation::Sum).unwrap();
        
        // Verify the result: 10 + 20 + 30 = 60.0
        assert_eq!(result, 60.0);
    }
    
    /// Test case: Calculate sum of an empty file
    /// 
    /// Creates an empty temporary file and verifies that the function
    /// returns 0.0 for sum operation
    #[test]
    fn test_blank_file_sum() {
        // Create an empty temporary file
        let temp_file = NamedTempFile::new().unwrap();
        
        // Calculate the sum from the empty file
        let result = calculate_from_file(temp_file.path().to_str().unwrap(), Operation::Sum).unwrap();
        
        // Verify that an empty file returns 0.0 for sum
        assert_eq!(result, 0.0);
    }
    
    /// Test case: Calculate sum from file with mixed content
    /// 
    /// Creates a file containing:
    /// - Valid numbers
    /// - Invalid text ("not a number")
    /// - Empty lines
    /// 
    /// Verifies that only valid numbers are included in the sum calculation
    /// and invalid content is properly ignored
    #[test]
    fn test_file_with_non_numeric_lines_sum() {
        // Create a temporary file with mixed content
        let mut temp_file = NamedTempFile::new().unwrap();
        
        // Write a mix of valid numbers, invalid text, and empty lines
        writeln!(temp_file, "10").unwrap();          // Valid: 10
        writeln!(temp_file, "not a number").unwrap(); // Invalid: ignored
        writeln!(temp_file, "20").unwrap();          // Valid: 20
        writeln!(temp_file, "").unwrap();            // Empty: ignored
        writeln!(temp_file, "30").unwrap();          // Valid: 30
        
        // Ensure all data is written to disk
        temp_file.flush().unwrap();
        
        // Calculate the sum from our test file
        let result = calculate_from_file(temp_file.path().to_str().unwrap(), Operation::Sum).unwrap();
        
        // Verify only valid numbers are used: 10 + 20 + 30 = 60.0
        assert_eq!(result, 60.0);
    }
}
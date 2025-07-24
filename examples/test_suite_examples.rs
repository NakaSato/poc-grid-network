//! # Example: Running the comprehensive test suite
//! 
//! This example demonstrates how to run various test categories

use std::process::Command;

fn main() {
    println!("Thai Energy Trading Blockchain - Test Suite Examples");
    println!("====================================================");
    
    // Example 1: Run all unit tests
    println!("\n1. Running Unit Tests:");
    run_test_command(&["unit"]);
    
    // Example 2: Run integration tests
    println!("\n2. Running Integration Tests:");
    run_test_command(&["integration"]);
    
    // Example 3: Generate coverage report
    println!("\n3. Generating Coverage Report:");
    run_test_command(&["coverage"]);
    
    // Example 4: Run performance benchmarks
    println!("\n4. Running Performance Tests:");
    run_test_command(&["performance"]);
    
    // Example 5: Run property-based tests
    println!("\n5. Running Property-Based Tests:");
    run_test_command(&["property"]);
    
    println!("\nTest suite examples completed!");
}

fn run_test_command(args: &[&str]) {
    let output = Command::new("./run_tests.sh")
        .args(args)
        .output()
        .expect("Failed to execute test script");
    
    if output.status.success() {
        println!("✓ {} tests passed", args[0]);
    } else {
        println!("✗ {} tests failed", args[0]);
        println!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }
}

#[cfg(test)]
mod test_examples {
    use super::*;
    
    #[test]
    fn test_example_compilation() {
        // This test ensures the example compiles correctly
        assert!(true);
    }
}

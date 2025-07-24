#!/bin/bash

# Comprehensive Test Runner for Thai Energy Trading Blockchain
# This script runs all test suites with proper setup and reporting

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test configuration
TEST_DB_URL="postgres://test_user:test_pass@localhost:5432/test_db"
RUST_LOG="debug"
CARGO_TARGET_DIR="./target/test"

# Functions
print_header() {
    echo -e "${BLUE}=================================="
    echo -e "$1"
    echo -e "==================================${NC}"
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

print_error() {
    echo -e "${RED}✗ $1${NC}"
}

# Setup test environment
setup_test_environment() {
    print_header "Setting up test environment"
    
    # Set environment variables
    export RUST_LOG=$RUST_LOG
    export DATABASE_URL=$TEST_DB_URL
    export RUST_BACKTRACE=1
    export CARGO_TARGET_DIR=$CARGO_TARGET_DIR
    
    # Create target directory
    mkdir -p $CARGO_TARGET_DIR
    
    # Start test containers if needed
    if command -v docker-compose &> /dev/null; then
        print_warning "Starting test database containers..."
        docker-compose -f docker-compose.test.yml up -d postgres redis
        sleep 5
    else
        print_warning "Docker Compose not found. Assuming external test database."
    fi
    
    print_success "Test environment ready"
}

# Cleanup test environment
cleanup_test_environment() {
    print_header "Cleaning up test environment"
    
    # Stop test containers
    if command -v docker-compose &> /dev/null; then
        docker-compose -f docker-compose.test.yml down
    fi
    
    print_success "Cleanup complete"
}

# Run unit tests
run_unit_tests() {
    print_header "Running Unit Tests"
    
    echo "Running CDA unit tests..."
    cargo test --test unit_cda_tests --features test-utils -- --nocapture
    
    echo "Running database unit tests..."
    cargo test --test unit_database_tests --features test-utils -- --nocapture
    
    echo "Running crypto unit tests..."
    cargo test crypto:: --lib --features test-utils -- --nocapture
    
    print_success "Unit tests completed"
}

# Run integration tests
run_integration_tests() {
    print_header "Running Integration Tests"
    
    echo "Running CDA-Database integration tests..."
    cargo test --test integration_cda_database --features test-utils -- --nocapture
    
    echo "Running full trading flow tests..."
    cargo test --test integration_trading_flow --features test-utils -- --nocapture
    
    echo "Running system integration tests..."
    cargo test --test integration_tests --features test-utils -- --nocapture
    
    print_success "Integration tests completed"
}

# Run performance tests
run_performance_tests() {
    print_header "Running Performance Tests"
    
    echo "Running CDA performance benchmarks..."
    cargo test --release --test performance_cda --features test-utils -- --nocapture --ignored
    
    echo "Running database performance tests..."
    cargo test --release --test performance_database --features test-utils -- --nocapture --ignored
    
    print_success "Performance tests completed"
}

# Run property-based tests
run_property_tests() {
    print_header "Running Property-Based Tests"
    
    echo "Running order matching property tests..."
    cargo test --test property_order_matching --features test-utils,proptest -- --nocapture
    
    echo "Running database consistency property tests..."
    cargo test --test property_database --features test-utils,proptest -- --nocapture
    
    print_success "Property-based tests completed"
}

# Run load tests
run_load_tests() {
    print_header "Running Load Tests"
    
    echo "Running concurrent user simulation..."
    cargo test --release --test load_concurrent_users --features test-utils -- --nocapture --ignored
    
    echo "Running high throughput tests..."
    cargo test --release --test load_high_throughput --features test-utils -- --nocapture --ignored
    
    print_success "Load tests completed"
}

# Generate test coverage report
generate_coverage_report() {
    print_header "Generating Test Coverage Report"
    
    if command -v cargo-tarpaulin &> /dev/null; then
        cargo tarpaulin \
            --out Html \
            --output-dir coverage \
            --timeout 300 \
            --features test-utils \
            --exclude-files "tests/*" "examples/*" "target/*"
        
        print_success "Coverage report generated in coverage/tarpaulin-report.html"
    else
        print_warning "cargo-tarpaulin not installed. Install with: cargo install cargo-tarpaulin"
    fi
}

# Generate benchmark report
generate_benchmark_report() {
    print_header "Running Criterion Benchmarks"
    
    if [ -d "benches" ]; then
        cargo bench --features test-utils
        print_success "Benchmark report generated in target/criterion/"
    else
        print_warning "No benchmark directory found"
    fi
}

# Validate test data integrity
validate_test_data() {
    print_header "Validating Test Data Integrity"
    
    echo "Checking test fixture consistency..."
    cargo test --test validate_test_fixtures --features test-utils -- --nocapture
    
    echo "Validating database schema..."
    cargo test --test validate_database_schema --features test-utils -- --nocapture
    
    print_success "Test data validation completed"
}

# Main execution logic
main() {
    local test_type=${1:-"all"}
    local start_time=$(date +%s)
    
    print_header "Thai Energy Trading Blockchain - Comprehensive Test Suite"
    echo "Test type: $test_type"
    echo "Started at: $(date)"
    echo ""
    
    # Trap cleanup on exit
    trap cleanup_test_environment EXIT
    
    # Setup
    setup_test_environment
    
    # Run tests based on type
    case $test_type in
        "unit")
            run_unit_tests
            ;;
        "integration")
            run_integration_tests
            ;;
        "performance")
            run_performance_tests
            ;;
        "property")
            run_property_tests
            ;;
        "load")
            run_load_tests
            ;;
        "coverage")
            run_unit_tests
            run_integration_tests
            generate_coverage_report
            ;;
        "benchmark")
            generate_benchmark_report
            ;;
        "validate")
            validate_test_data
            ;;
        "all")
            validate_test_data
            run_unit_tests
            run_integration_tests
            run_property_tests
            run_performance_tests
            generate_coverage_report
            ;;
        *)
            echo "Usage: $0 [unit|integration|performance|property|load|coverage|benchmark|validate|all]"
            exit 1
            ;;
    esac
    
    # Summary
    local end_time=$(date +%s)
    local duration=$((end_time - start_time))
    
    print_header "Test Suite Summary"
    echo "Total execution time: ${duration}s"
    echo "Completed at: $(date)"
    
    if [ $? -eq 0 ]; then
        print_success "All tests passed successfully!"
        exit 0
    else
        print_error "Some tests failed. Check the output above."
        exit 1
    fi
}

# Check if script is being sourced or executed
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi

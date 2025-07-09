# NetDrop Testing Documentation

This document describes the comprehensive test suite for the NetDrop upload and download functionality.

## Test Structure

The test suite is organized into three main categories:

### 1. Unit Tests (`tests/unit_tests.rs`)

Tests individual components and functions in isolation:

- **Hash Generation Tests**

  - `test_hash_generation_consistency`: Verifies SHA256 hash consistency
  - `test_hash_generation_different_data`: Ensures different data produces different hashes
  - `test_hash_generation_empty_data`: Tests hash generation for empty data

- **File Operations Tests**

  - `test_file_name_generation`: Tests filename generation from hash
  - `test_directory_creation`: Verifies directory creation functionality
  - `test_file_write_and_read`: Tests basic file I/O operations
  - `test_file_size_calculation`: Validates file size calculations

- **Environment and Configuration Tests**

  - `test_environment_variable_handling`: Tests DATA_DIR environment variable handling
  - `test_upload_path_construction`: Verifies correct path construction

- **Data Handling Tests**
  - `test_binary_data_handling`: Tests binary data processing
  - `test_large_file_handling`: Tests handling of large files (1MB)
  - `test_content_type_handling`: Tests various content types

### 2. Integration Tests (`tests/integration_tests.rs`)

Tests complete workflows and component interactions:

- **File Operations Integration**

  - `test_file_operations_integration`: Complete upload/download simulation
  - `test_different_file_types_integration`: Tests various file types
  - `test_multiple_file_operations`: Tests handling multiple files

- **System Integration**
  - `test_file_persistence`: Verifies file persistence across operations
  - `test_hash_consistency`: Tests hash consistency in workflows
  - `test_data_dir_environment_integration`: Tests custom DATA_DIR configuration

### 3. Library Tests (`src/tests.rs`)

Tests core library functionality:

- **Upload Logic Tests**

  - `test_upload_file_logic`: Tests complete upload process simulation
  - `test_empty_file_handling`: Tests empty file detection
  - `test_large_file_handling`: Tests large file upload logic
  - `test_directory_creation`: Tests upload directory creation
  - `test_hash_consistency`: Tests hash generation consistency

- **Download Logic Tests**

  - `test_download_simulation`: Tests download process simulation
  - `test_binary_data_handling`: Tests binary data upload/download

- **Database Tests**
  - `test_create_file_in_database`: Tests file record creation
  - `test_get_file_by_hash_existing`: Tests file retrieval by hash
  - `test_get_file_by_hash_nonexistent`: Tests handling of non-existent files
  - `test_create_multiple_files`: Tests multiple file database operations

## Running Tests

### Run All Tests

```bash
cargo test
```

### Run Specific Test Categories

```bash
# Unit tests only
cargo test --test unit_tests

# Integration tests only
cargo test --test integration_tests

# Library tests only
cargo test --lib
```

### Run Tests with Output

```bash
cargo test -- --nocapture
```

### Run Tests Serially (for environment variable tests)

```bash
cargo test -- --test-threads=1
```

## Test Features

### Environment Safety

- Tests use temporary directories to avoid conflicts
- Environment variables are safely managed with unsafe blocks
- Tests are marked with `#[serial]` where needed to prevent race conditions

### Comprehensive Coverage

- **File Operations**: Upload, download, hash generation, file I/O
- **Data Types**: Text, binary, JSON, large files, empty files
- **Error Handling**: Non-existent files, invalid paths, empty data
- **Configuration**: Environment variables, custom directories
- **Database**: File metadata storage and retrieval

### Test Data

- Various file sizes: empty, small (bytes), medium (KB), large (1MB)
- Different content types: text, binary, JSON, XML
- Edge cases: empty files, special characters, binary data

## Dependencies

The test suite uses these additional dependencies:

- `tempfile`: For creating temporary test directories
- `serial_test`: For serializing tests that modify global state
- `serde_json`: For JSON handling in tests

## Test Philosophy

The tests follow these principles:

1. **Isolation**: Each test is independent and doesn't affect others
2. **Determinism**: Tests produce consistent results across runs
3. **Comprehensiveness**: Cover normal cases, edge cases, and error conditions
4. **Realism**: Simulate real-world usage patterns
5. **Performance**: Tests run quickly to enable frequent execution

## Continuous Integration

These tests are designed to run in CI environments and provide:

- Fast execution (typically under 1 second)
- No external dependencies
- Deterministic results
- Clear failure messages
- Comprehensive coverage reporting

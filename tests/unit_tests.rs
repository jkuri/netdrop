use sha2::{Sha256, Digest};
use hex;
use std::fs;
use tempfile::TempDir;
use std::env;

#[test]
fn test_hash_generation_consistency() {
    let test_data = b"Test data for hash generation";

    // Generate hash twice
    let mut hasher1 = Sha256::new();
    hasher1.update(test_data);
    let hash1 = hex::encode(hasher1.finalize());

    let mut hasher2 = Sha256::new();
    hasher2.update(test_data);
    let hash2 = hex::encode(hasher2.finalize());

    assert_eq!(hash1, hash2);
    assert_eq!(hash1.len(), 64); // SHA256 produces 64 character hex string
}

#[test]
fn test_hash_generation_different_data() {
    let data1 = b"First test data";
    let data2 = b"Second test data";

    let mut hasher1 = Sha256::new();
    hasher1.update(data1);
    let hash1 = hex::encode(hasher1.finalize());

    let mut hasher2 = Sha256::new();
    hasher2.update(data2);
    let hash2 = hex::encode(hasher2.finalize());

    assert_ne!(hash1, hash2);
}

#[test]
fn test_hash_generation_empty_data() {
    let empty_data = b"";

    let mut hasher = Sha256::new();
    hasher.update(empty_data);
    let hash = hex::encode(hasher.finalize());

    // SHA256 of empty string should be a known value
    assert_eq!(hash, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
}

#[test]
fn test_file_name_generation() {
    let test_hash = "abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890";
    let file_name = &test_hash[..16]; // First 16 chars

    assert_eq!(file_name, "abcdef1234567890");
    assert_eq!(file_name.len(), 16);
}

#[test]
fn test_directory_creation() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let test_upload_dir = temp_dir.path().join("test_uploads");

    // Directory should not exist initially
    assert!(!test_upload_dir.exists());

    // Create directory
    fs::create_dir_all(&test_upload_dir).expect("Failed to create directory");

    // Directory should now exist
    assert!(test_upload_dir.exists());
    assert!(test_upload_dir.is_dir());
}

#[test]
fn test_file_write_and_read() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let test_file_path = temp_dir.path().join("test_file.bin");
    let test_data = b"Test file content for write/read test";

    // Write file
    fs::write(&test_file_path, test_data).expect("Failed to write file");

    // Verify file exists
    assert!(test_file_path.exists());
    assert!(test_file_path.is_file());

    // Read file back
    let read_data = fs::read(&test_file_path).expect("Failed to read file");
    assert_eq!(read_data, test_data);
}

#[test]
fn test_file_size_calculation() {
    let test_data = b"Size calculation test data";
    let expected_size = test_data.len();

    assert_eq!(expected_size, 26);

    // Test with different sizes
    let small_data = b"small";
    assert_eq!(small_data.len(), 5);

    let large_data = vec![0u8; 1024];
    assert_eq!(large_data.len(), 1024);
}

#[test]
fn test_environment_variable_handling() {
    // Test default value when env var is not set
    unsafe {
        env::remove_var("TEST_DATA_DIR");
    }
    let data_dir = env::var("TEST_DATA_DIR").unwrap_or_else(|_| "default_data".to_string());
    assert_eq!(data_dir, "default_data");

    // Test when env var is set
    unsafe {
        env::set_var("TEST_DATA_DIR", "custom_data");
    }
    let data_dir = env::var("TEST_DATA_DIR").unwrap_or_else(|_| "default_data".to_string());
    assert_eq!(data_dir, "custom_data");

    // Clean up
    unsafe {
        env::remove_var("TEST_DATA_DIR");
    }
}

#[test]
fn test_upload_path_construction() {
    let data_dir = "test_data";
    let upload_dir = format!("{}/uploads", data_dir);
    assert_eq!(upload_dir, "test_data/uploads");

    let file_hash = "abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890";
    let file_name = &file_hash[..16];
    let file_path = format!("{}/{}", upload_dir, file_name);
    assert_eq!(file_path, "test_data/uploads/abcdef1234567890");
}

#[test]
fn test_binary_data_handling() {
    // Test with various binary data patterns
    let binary_data: Vec<u8> = (0..=255).collect();
    assert_eq!(binary_data.len(), 256);
    assert_eq!(binary_data[0], 0);
    assert_eq!(binary_data[255], 255);

    // Test hash generation with binary data
    let mut hasher = Sha256::new();
    hasher.update(&binary_data);
    let hash = hex::encode(hasher.finalize());
    assert_eq!(hash.len(), 64);

    // Test file operations with binary data
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let binary_file_path = temp_dir.path().join("binary_test.bin");

    fs::write(&binary_file_path, &binary_data).expect("Failed to write binary file");
    let read_binary_data = fs::read(&binary_file_path).expect("Failed to read binary file");
    assert_eq!(read_binary_data, binary_data);
}

#[test]
fn test_large_file_handling() {
    // Test with a moderately large file (1MB)
    let large_data = vec![0xAB; 1024 * 1024]; // 1MB of 0xAB bytes
    assert_eq!(large_data.len(), 1024 * 1024);

    // Test hash generation
    let mut hasher = Sha256::new();
    hasher.update(&large_data);
    let hash = hex::encode(hasher.finalize());
    assert_eq!(hash.len(), 64);

    // Test file operations
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let large_file_path = temp_dir.path().join("large_test.bin");

    fs::write(&large_file_path, &large_data).expect("Failed to write large file");
    let read_large_data = fs::read(&large_file_path).expect("Failed to read large file");
    assert_eq!(read_large_data.len(), 1024 * 1024);
    assert_eq!(read_large_data, large_data);
}

#[test]
fn test_content_type_handling() {
    // Test various content types that might be uploaded
    let text_content = "Hello, World!".as_bytes();
    let json_content = r#"{"key": "value"}"#.as_bytes();
    let xml_content = r#"<?xml version="1.0"?><root></root>"#.as_bytes();

    // All should be handled as binary data
    assert!(!text_content.is_empty());
    assert!(!json_content.is_empty());
    assert!(!xml_content.is_empty());

    // Generate hashes for each
    let mut hasher = Sha256::new();
    hasher.update(text_content);
    let text_hash = hex::encode(hasher.finalize());

    let mut hasher = Sha256::new();
    hasher.update(json_content);
    let json_hash = hex::encode(hasher.finalize());

    let mut hasher = Sha256::new();
    hasher.update(xml_content);
    let xml_hash = hex::encode(hasher.finalize());

    // All hashes should be different
    assert_ne!(text_hash, json_hash);
    assert_ne!(text_hash, xml_hash);
    assert_ne!(json_hash, xml_hash);
}

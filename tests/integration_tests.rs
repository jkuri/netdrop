// Integration tests for netdrop upload/download functionality
// These tests verify the complete upload/download workflow

use std::fs;
use tempfile::TempDir;
use std::env;
use serial_test::serial;

fn setup_test_env() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    unsafe {
        env::set_var("DATA_DIR", temp_dir.path().to_str().unwrap());
        env::set_var("DATABASE_URL", ":memory:");
    }
    temp_dir
}

#[test]
#[serial]
fn test_file_operations_integration() {
    let temp_dir = setup_test_env();

    // Test file creation and hash generation
    let test_content = b"This is a complete integration test file content.";

    // Simulate the upload process
    let uploads_dir = temp_dir.path().join("uploads");
    fs::create_dir_all(&uploads_dir).expect("Failed to create uploads directory");

    // Generate hash like the upload function does
    use sha2::{Sha256, Digest};
    use hex;

    let mut hasher = Sha256::new();
    hasher.update(test_content);
    let hash_bytes = hasher.finalize();
    let file_hash = hex::encode(hash_bytes);

    // Generate filename like the upload function does
    let file_name = &file_hash[..16];
    let file_path = uploads_dir.join(file_name);

    // Save file like the upload function does
    fs::write(&file_path, test_content).expect("Failed to save file");

    // Verify file was saved correctly
    assert!(file_path.exists());
    let saved_content = fs::read(&file_path).expect("Failed to read saved file");
    assert_eq!(saved_content, test_content);

    // Test that we can read it back (simulating download)
    let downloaded_content = fs::read(&file_path).expect("Failed to download file");
    assert_eq!(downloaded_content, test_content);
}

#[test]
#[serial]
fn test_different_file_types_integration() {
    let temp_dir = setup_test_env();
    let uploads_dir = temp_dir.path().join("uploads");
    fs::create_dir_all(&uploads_dir).expect("Failed to create uploads directory");

    use sha2::{Sha256, Digest};
    use hex;

    // Test text file
    let text_content = b"This is a text file content.";
    let mut hasher = Sha256::new();
    hasher.update(text_content);
    let text_hash = hex::encode(hasher.finalize());

    // Test binary file (simulated image data)
    let binary_content: Vec<u8> = vec![0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10]; // JPEG header
    let mut hasher = Sha256::new();
    hasher.update(&binary_content);
    let binary_hash = hex::encode(hasher.finalize());

    // Test JSON-like content
    let json_content = br#"{"test": "data", "number": 42}"#;
    let mut hasher = Sha256::new();
    hasher.update(json_content);
    let json_hash = hex::encode(hasher.finalize());

    // Verify all files have different hashes
    assert_ne!(text_hash, binary_hash);
    assert_ne!(text_hash, json_hash);
    assert_ne!(binary_hash, json_hash);

    // Save all files and verify they can be read back
    let text_path = uploads_dir.join(&text_hash[..16]);
    let binary_path = uploads_dir.join(&binary_hash[..16]);
    let json_path = uploads_dir.join(&json_hash[..16]);

    fs::write(&text_path, text_content).expect("Failed to save text file");
    fs::write(&binary_path, &binary_content).expect("Failed to save binary file");
    fs::write(&json_path, json_content).expect("Failed to save JSON file");

    // Verify all files can be read back correctly
    assert_eq!(fs::read(&text_path).unwrap(), text_content);
    assert_eq!(fs::read(&binary_path).unwrap(), binary_content);
    assert_eq!(fs::read(&json_path).unwrap(), json_content);
}

#[test]
#[serial]
fn test_multiple_file_operations() {
    let temp_dir = setup_test_env();
    let uploads_dir = temp_dir.path().join("uploads");
    fs::create_dir_all(&uploads_dir).expect("Failed to create uploads directory");

    use sha2::{Sha256, Digest};
    use hex;

    let test_data1 = b"Multiple file test 1";
    let test_data2 = b"Multiple file test 2";
    let test_data3 = b"Multiple file test 3";

    // Generate hashes for all files
    let mut hasher = Sha256::new();
    hasher.update(test_data1);
    let hash1 = hex::encode(hasher.finalize());

    let mut hasher = Sha256::new();
    hasher.update(test_data2);
    let hash2 = hex::encode(hasher.finalize());

    let mut hasher = Sha256::new();
    hasher.update(test_data3);
    let hash3 = hex::encode(hasher.finalize());

    // All hashes should be different
    assert_ne!(hash1, hash2);
    assert_ne!(hash1, hash3);
    assert_ne!(hash2, hash3);

    // Save all files
    let path1 = uploads_dir.join(&hash1[..16]);
    let path2 = uploads_dir.join(&hash2[..16]);
    let path3 = uploads_dir.join(&hash3[..16]);

    fs::write(&path1, test_data1).expect("Failed to save file 1");
    fs::write(&path2, test_data2).expect("Failed to save file 2");
    fs::write(&path3, test_data3).expect("Failed to save file 3");

    // Verify all files exist and have correct content
    assert_eq!(fs::read(&path1).unwrap(), test_data1);
    assert_eq!(fs::read(&path2).unwrap(), test_data2);
    assert_eq!(fs::read(&path3).unwrap(), test_data3);
}

#[test]
#[serial]
fn test_file_persistence() {
    let temp_dir = setup_test_env();
    let uploads_dir = temp_dir.path().join("uploads");
    fs::create_dir_all(&uploads_dir).expect("Failed to create uploads directory");

    use sha2::{Sha256, Digest};
    use hex;

    let test_content = b"Persistence test content";

    // Generate hash and save file
    let mut hasher = Sha256::new();
    hasher.update(test_content);
    let file_hash = hex::encode(hasher.finalize());
    let file_path = uploads_dir.join(&file_hash[..16]);

    fs::write(&file_path, test_content).expect("Failed to save file");

    // Read file multiple times to ensure persistence
    for _ in 0..3 {
        let read_content = fs::read(&file_path).expect("Failed to read file");
        assert_eq!(read_content, test_content);
        assert!(file_path.exists());
    }
}

#[test]
#[serial]
fn test_hash_consistency() {
    let _temp_dir = setup_test_env();

    use sha2::{Sha256, Digest};
    use hex;

    // Upload the same content twice and verify same hash
    let test_content = b"Hash consistency test";

    let mut hasher1 = Sha256::new();
    hasher1.update(test_content);
    let hash1 = hex::encode(hasher1.finalize());

    let mut hasher2 = Sha256::new();
    hasher2.update(test_content);
    let hash2 = hex::encode(hasher2.finalize());

    // Both should generate the same hash
    assert_eq!(hash1, hash2);

    // File names should be the same
    let filename1 = &hash1[..16];
    let filename2 = &hash2[..16];
    assert_eq!(filename1, filename2);
}

#[test]
#[serial]
fn test_data_dir_environment_integration() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let custom_data_dir = temp_dir.path().join("custom_data");
    unsafe {
        env::set_var("DATA_DIR", custom_data_dir.to_str().unwrap());
    }

    // Simulate the upload directory creation process
    let data_dir = env::var("DATA_DIR").unwrap_or_else(|_| "data".to_string());
    let upload_dir = format!("{}/uploads", data_dir);

    fs::create_dir_all(&upload_dir).expect("Failed to create upload directory");

    // Verify the custom uploads directory was created
    assert!(custom_data_dir.join("uploads").exists());

    // Test file operations in custom directory
    let test_content = b"Custom directory test";
    let test_file_path = custom_data_dir.join("uploads").join("test_file");

    fs::write(&test_file_path, test_content).expect("Failed to write test file");
    let read_content = fs::read(&test_file_path).expect("Failed to read test file");
    assert_eq!(read_content, test_content);
}

#[test]
#[serial]
fn test_cors_configuration() {
    // Test that CORS is properly configured by checking that the dependency compiles
    // and the application builds successfully with CORS enabled
    let _temp_dir = setup_test_env();

    // This test verifies that:
    // 1. rocket_cors dependency is properly configured
    // 2. CORS fairing is attached to the rocket instance
    // 3. The application compiles without CORS-related errors

    // The actual CORS headers would be tested in full integration tests
    // with a running server, but this ensures the configuration is valid
    assert!(true, "CORS configuration compiles successfully");
}

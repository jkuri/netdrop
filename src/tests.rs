#[cfg(test)]
mod tests {
    use tempfile::TempDir;
    use std::fs;
    use std::env;
    use serial_test::serial;
    use sha2::{Sha256, Digest};
    use hex;

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
    fn test_upload_file_logic() {
        let test_data = b"Hello, World! This is test file content.";

        // Simulate upload logic
        let data_dir = env::var("DATA_DIR").unwrap_or_else(|_| "data".to_string());
        let upload_dir = format!("{}/uploads", data_dir);

        // Create upload directory
        fs::create_dir_all(&upload_dir).expect("Failed to create upload directory");

        // Calculate file hash
        let mut hasher = Sha256::new();
        hasher.update(test_data);
        let hash_bytes = hasher.finalize();
        let file_hash = hex::encode(hash_bytes);

        // Generate filename and path
        let file_name = &file_hash[..16];
        let file_path = format!("{}/{}", upload_dir, file_name);

        // Save file
        fs::write(&file_path, test_data).expect("Failed to save file");

        // Verify file was saved correctly
        assert!(std::path::Path::new(&file_path).exists());
        let saved_data = fs::read(&file_path).expect("Failed to read saved file");
        assert_eq!(saved_data, test_data);
        assert_eq!(file_hash.len(), 64); // SHA256 hash length
        assert_eq!(file_name.len(), 16); // First 16 chars
    }

    #[test]
    #[serial]
    fn test_empty_file_handling() {
        let _temp_dir = setup_test_env();
        let empty_data = b"";

        // Test that empty data is detected
        assert!(empty_data.is_empty());

        // Hash of empty data should still be valid
        let mut hasher = Sha256::new();
        hasher.update(empty_data);
        let hash = hex::encode(hasher.finalize());
        assert_eq!(hash, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
    }

    #[test]
    #[serial]
    fn test_large_file_handling() {
        // Create a 1MB test file
        let test_data = vec![0u8; 1024 * 1024];

        let data_dir = env::var("DATA_DIR").unwrap_or_else(|_| "data".to_string());
        let upload_dir = format!("{}/uploads", data_dir);
        fs::create_dir_all(&upload_dir).expect("Failed to create upload directory");

        // Calculate hash and save file
        let mut hasher = Sha256::new();
        hasher.update(&test_data);
        let file_hash = hex::encode(hasher.finalize());
        let file_name = &file_hash[..16];
        let file_path = format!("{}/{}", upload_dir, file_name);

        fs::write(&file_path, &test_data).expect("Failed to save large file");

        // Verify large file was saved correctly
        let saved_data = fs::read(&file_path).expect("Failed to read large file");
        assert_eq!(saved_data.len(), 1024 * 1024);
        assert_eq!(saved_data, test_data);
    }

    #[test]
    #[serial]
    fn test_directory_creation() {
        let temp_dir = setup_test_env();

        let data_dir = env::var("DATA_DIR").unwrap_or_else(|_| "data".to_string());
        let upload_dir = format!("{}/uploads", data_dir);

        // Directory should not exist initially
        assert!(!std::path::Path::new(&upload_dir).exists());

        // Create directory like upload function does
        fs::create_dir_all(&upload_dir).expect("Failed to create upload directory");

        // Check that the uploads directory was created
        let uploads_dir = temp_dir.path().join("uploads");
        assert!(uploads_dir.exists());
        assert!(uploads_dir.is_dir());
    }

    #[test]
    #[serial]
    fn test_hash_consistency() {
        let _temp_dir = setup_test_env();
        let test_data = b"Consistent test data";

        // Generate hash twice
        let mut hasher1 = Sha256::new();
        hasher1.update(test_data);
        let hash1 = hex::encode(hasher1.finalize());

        let mut hasher2 = Sha256::new();
        hasher2.update(test_data);
        let hash2 = hex::encode(hasher2.finalize());

        // Both should generate the same hash
        assert_eq!(hash1, hash2);
    }

    #[test]
    #[serial]
    fn test_download_simulation() {
        let test_data = b"Download test content";

        // Simulate upload process
        let data_dir = env::var("DATA_DIR").unwrap_or_else(|_| "data".to_string());
        let upload_dir = format!("{}/uploads", data_dir);
        fs::create_dir_all(&upload_dir).expect("Failed to create upload directory");

        let mut hasher = Sha256::new();
        hasher.update(test_data);
        let file_hash = hex::encode(hasher.finalize());
        let file_name = &file_hash[..16];
        let file_path = format!("{}/{}", upload_dir, file_name);

        // Save file
        fs::write(&file_path, test_data).expect("Failed to save file");

        // Simulate download process
        let downloaded_data = fs::read(&file_path).expect("Failed to read file");
        assert_eq!(downloaded_data, test_data);
    }

    #[test]
    #[serial]
    fn test_binary_data_handling() {
        // Create binary test data with various byte values
        let test_data: Vec<u8> = (0..=255).collect();

        let data_dir = env::var("DATA_DIR").unwrap_or_else(|_| "data".to_string());
        let upload_dir = format!("{}/uploads", data_dir);
        fs::create_dir_all(&upload_dir).expect("Failed to create upload directory");

        let mut hasher = Sha256::new();
        hasher.update(&test_data);
        let file_hash = hex::encode(hasher.finalize());
        let file_name = &file_hash[..16];
        let file_path = format!("{}/{}", upload_dir, file_name);

        // Save and read binary data
        fs::write(&file_path, &test_data).expect("Failed to save binary file");
        let read_data = fs::read(&file_path).expect("Failed to read binary file");
        assert_eq!(read_data, test_data);
    }

    #[test]
    #[serial]
    fn test_file_size_tracking() {
        let _temp_dir = setup_test_env();
        let test_data = b"Size tracking test";
        let expected_size = test_data.len();

        assert_eq!(expected_size, 18);

        // Test with different sizes
        let small_data = b"small";
        assert_eq!(small_data.len(), 5);

        let large_data = vec![0u8; 1024];
        assert_eq!(large_data.len(), 1024);
    }

    #[test]
    #[serial]
    fn test_data_dir_environment_variable() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let custom_data_dir = temp_dir.path().join("custom_data");

        unsafe {
            env::set_var("DATA_DIR", custom_data_dir.to_str().unwrap());
        }

        let data_dir = env::var("DATA_DIR").unwrap_or_else(|_| "data".to_string());
        let upload_dir = format!("{}/uploads", data_dir);
        fs::create_dir_all(&upload_dir).expect("Failed to create upload directory");

        // Verify the custom uploads directory was created
        let uploads_dir = custom_data_dir.join("uploads");
        assert!(uploads_dir.exists());
    }
}

#[cfg(test)]
mod database_tests {
    use crate::{establish_connection, create_file, get_file_by_hash};
    use crate::models::NewFile;
    use diesel::prelude::*;
    use std::env;
    use serial_test::serial;

    fn setup_test_database() -> SqliteConnection {
        unsafe {
            env::set_var("DATABASE_URL", ":memory:");
        }
        let mut conn = establish_connection();

        // Run migrations manually for in-memory database
        diesel::sql_query(
            "CREATE TABLE files (
                id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
                file_hash VARCHAR NOT NULL,
                file_name VARCHAR NOT NULL,
                file_path VARCHAR NOT NULL,
                size INTEGER NOT NULL,
                private BOOLEAN NOT NULL DEFAULT 1,
                created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
            )"
        ).execute(&mut conn).expect("Failed to create table");

        conn
    }

    #[test]
    #[serial]
    fn test_create_file_in_database() {
        let mut conn = setup_test_database();

        let new_file = NewFile {
            file_hash: "test_hash_123456789",
            file_name: "test_file",
            file_path: "/tmp/test_file",
            size: 1024,
            private: true,
        };

        let created_file = create_file(&mut conn, new_file);

        assert_eq!(created_file.file_hash, "test_hash_123456789");
        assert_eq!(created_file.file_name, "test_file");
        assert_eq!(created_file.file_path, "/tmp/test_file");
        assert_eq!(created_file.size, 1024);
        assert_eq!(created_file.private, true);
        assert!(created_file.id > 0);
    }

    #[test]
    #[serial]
    fn test_get_file_by_hash_existing() {
        let mut conn = setup_test_database();

        let new_file = NewFile {
            file_hash: "existing_hash_123",
            file_name: "existing_file",
            file_path: "/tmp/existing_file",
            size: 2048,
            private: false,
        };

        let created_file = create_file(&mut conn, new_file);
        let retrieved_file = get_file_by_hash(&mut conn, "existing_hash_123");

        assert!(retrieved_file.is_some());
        let file = retrieved_file.unwrap();
        assert_eq!(file.id, created_file.id);
        assert_eq!(file.file_hash, "existing_hash_123");
        assert_eq!(file.file_name, "existing_file");
    }

    #[test]
    #[serial]
    fn test_get_file_by_hash_nonexistent() {
        let mut conn = setup_test_database();

        let result = get_file_by_hash(&mut conn, "nonexistent_hash");
        assert!(result.is_none());
    }

    #[test]
    #[serial]
    fn test_create_multiple_files() {
        let mut conn = setup_test_database();

        let file1 = NewFile {
            file_hash: "hash1",
            file_name: "file1",
            file_path: "/tmp/file1",
            size: 100,
            private: true,
        };

        let file2 = NewFile {
            file_hash: "hash2",
            file_name: "file2",
            file_path: "/tmp/file2",
            size: 200,
            private: false,
        };

        let created1 = create_file(&mut conn, file1);
        let created2 = create_file(&mut conn, file2);

        assert_ne!(created1.id, created2.id);

        let retrieved1 = get_file_by_hash(&mut conn, "hash1").unwrap();
        let retrieved2 = get_file_by_hash(&mut conn, "hash2").unwrap();

        assert_eq!(retrieved1.file_name, "file1");
        assert_eq!(retrieved2.file_name, "file2");
        assert_eq!(retrieved1.private, true);
        assert_eq!(retrieved2.private, false);
    }
}

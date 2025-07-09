#[macro_use] extern crate rocket;

use rocket::response::content::RawHtml;
use include_dir::{include_dir, Dir};
use rocket::http::ContentType;
use std::path::PathBuf;
use rocket::serde::{Serialize, json::Json};
use rocket::data::{Data, ToByteUnit};
use multer::Multipart;
use tokio_util::io::ReaderStream;
use sha2::{Sha256, Digest};
use hex;
use std::fs;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

use netdrop::{establish_connection, create_file, get_file_by_hash, models::NewFile};
use rocket::response::Responder;
use rocket::http::{Header, Status};
use rocket_cors::{AllowedOrigins, CorsOptions};

static ASSETS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/web/netdrop/dist");

#[derive(Serialize)]
pub struct UploadResponse {
    success: bool,
    message: String,
    file_id: Option<i32>,
    file_hash: Option<String>,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    success: bool,
    error: String,
}

#[get("/<file..>")]
pub async fn static_files(file: PathBuf) -> Option<(ContentType, Vec<u8>)> {
    let path = file.display().to_string();
    let file_content = ASSETS.get_file(&path)?;
    let content_type = ContentType::from_extension(file.extension()?.to_str()?)?;

    Some((content_type, file_content.contents().to_vec()))
}

#[post("/api/v1/upload", data = "<data>", format = "multipart/form-data")]
pub async fn upload_file(content_type: &ContentType, data: Data<'_>) -> Result<Json<UploadResponse>, Json<ErrorResponse>> {
    // Extract boundary from content type
    let boundary = content_type
        .params()
        .find(|(name, _)| name == &"boundary")
        .map(|(_, value)| value)
        .ok_or_else(|| Json(ErrorResponse {
            success: false,
            error: "Missing boundary in multipart data".to_string(),
        }))?;

    // Read the data stream and convert to a format multer can use
    let stream = data.open(1000.megabytes());
    let reader_stream = ReaderStream::new(stream);
    let mut multipart = Multipart::new(reader_stream, boundary);

    let mut file_data: Option<Vec<u8>> = None;
    let mut filename: Option<String> = None;

    // Process multipart fields
    while let Some(field) = multipart.next_field().await.map_err(|_| Json(ErrorResponse {
        success: false,
        error: "Failed to parse multipart data".to_string(),
    }))? {

        let field_name = field.name().unwrap_or("").to_string();

        if field_name == "file" {
            filename = field.file_name().map(|s| s.to_string());
            file_data = Some(field.bytes().await.map_err(|_| Json(ErrorResponse {
                success: false,
                error: "Failed to read file data".to_string(),
            }))?.to_vec());
        }
    }

    let buffer = file_data.ok_or_else(|| Json(ErrorResponse {
        success: false,
        error: "No file data found in multipart upload".to_string(),
    }))?;

    let original_filename = filename.unwrap_or_else(|| "uploaded_file".to_string());

    process_file_upload(buffer, original_filename).await
}

async fn process_file_upload(buffer: Vec<u8>, original_filename: String) -> Result<Json<UploadResponse>, Json<ErrorResponse>> {
    // Get upload directory from environment variable, default to "uploads"
    let data_dir = env::var("DATA_DIR").unwrap_or_else(|_| "data".to_string());
    let upload_dir = format!("{}/uploads", data_dir);

    if let Err(_) = fs::create_dir_all(&upload_dir) {
        return Err(Json(ErrorResponse {
            success: false,
            error: "Failed to create upload directory".to_string(),
        }));
    }

    // Calculate file hash with timestamp to ensure uniqueness
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    let mut hasher = Sha256::new();
    hasher.update(&buffer);
    hasher.update(timestamp.to_be_bytes()); // Add timestamp to hash
    let hash_bytes = hasher.finalize();
    let file_hash = hex::encode(hash_bytes);

    // Use original filename for file_name, hash-based name for storage
    let short_hash = format!("{}", &file_hash[..16]); // Use first 16 chars of hash for storage
    let file_path = format!("{}/{}", upload_dir, short_hash);

    // Save file to disk
    if let Err(_) = fs::write(&file_path, &buffer) {
        return Err(Json(ErrorResponse {
            success: false,
            error: "Failed to save file to disk".to_string(),
        }));
    }

    // Save file info to database
    let mut connection = match establish_connection() {
        conn => conn,
    };

    let new_file = NewFile {
        file_hash: &short_hash,        // Store short hash for lookups
        file_name: &original_filename, // Use original filename
        file_path: &file_path,         // Use hash-based storage path
        size: buffer.len() as i32,
        private: true, // Default to private
    };

    // Use the create_file function from lib.rs
    let file = create_file(&mut connection, new_file);

    Ok(Json(UploadResponse {
        success: true,
        message: "File uploaded successfully".to_string(),
        file_id: Some(file.id),
        file_hash: Some(file.file_hash),
    }))
}

#[derive(Responder)]
pub struct FileDownload {
    inner: Vec<u8>,
    content_type: ContentType,
    content_disposition: Header<'static>,
}

#[get("/download/<file_hash>")]
pub fn download_file(file_hash: &str) -> Result<FileDownload, Status> {
    // Get file info from database
    let mut connection = establish_connection();
    let file = match get_file_by_hash(&mut connection, file_hash) {
        Some(file) => file,
        None => return Err(Status::NotFound),
    };

    // Read file from disk
    let file_content = match fs::read(&file.file_path) {
        Ok(content) => content,
        Err(_) => return Err(Status::InternalServerError),
    };

    // Return file with proper headers
    Ok(FileDownload {
        inner: file_content,
        content_type: ContentType::Binary,
        content_disposition: Header::new(
            "Content-Disposition",
            format!("attachment; filename=\"{}\"", file.file_name)
        ),
    })
}

#[get("/")]
pub fn index() -> RawHtml<&'static str> {
    RawHtml(ASSETS.get_file("index.html").map_or("Not found", |f| std::str::from_utf8(f.contents()).unwrap_or("Invalid UTF-8")))
}

#[launch]
pub fn rocket() -> _ {
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![rocket::http::Method::Get, rocket::http::Method::Post]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true)
        .to_cors()
        .expect("Error creating CORS fairing");

    rocket::build()
        .mount("/", routes![index, static_files, upload_file, download_file])
        .attach(cors)
}

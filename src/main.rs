#[macro_use] extern crate rocket;

use rocket::response::content::RawHtml;
use include_dir::{include_dir, Dir};
use rocket::http::ContentType;
use std::path::PathBuf;
use rocket::serde::{Serialize, json::Json};
use rocket::data::{Data, ToByteUnit};
use rocket::tokio::io::AsyncReadExt;
use sha2::{Sha256, Digest};
use hex;
use std::fs;
use std::env;

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

#[post("/api/v1/upload", data = "<data>")]
pub async fn upload_file(data: Data<'_>) -> Result<Json<UploadResponse>, Json<ErrorResponse>> {
    // Get upload directory from environment variable, default to "uploads"
    let data_dir = env::var("DATA_DIR").unwrap_or_else(|_| "data".to_string());
    let upload_dir = format!("{}/uploads", data_dir);

    if let Err(_) = fs::create_dir_all(&upload_dir) {
        return Err(Json(ErrorResponse {
            success: false,
            error: "Failed to create upload directory".to_string(),
        }));
    }

    // Read the uploaded data
    let mut buffer = Vec::new();
    let mut stream = data.open(1000.megabytes()); // Limit to 1GB

    if let Err(_) = stream.read_to_end(&mut buffer).await {
        return Err(Json(ErrorResponse {
            success: false,
            error: "Failed to read uploaded file".to_string(),
        }));
    }

    if buffer.is_empty() {
        return Err(Json(ErrorResponse {
            success: false,
            error: "No file data received".to_string(),
        }));
    }

    // Calculate file hash
    let mut hasher = Sha256::new();
    hasher.update(&buffer);
    let hash_bytes = hasher.finalize();
    let file_hash = hex::encode(hash_bytes);

    // Generate filename based on hash (without extension)
    let file_name = format!("{}", &file_hash[..16]); // Use first 16 chars of hash
    let file_path = format!("{}/{}", upload_dir, file_name);

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
        file_hash: &file_hash,
        file_name: &file_name,
        file_path: &file_path,
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
pub fn download_file(file_hash: String) -> Result<FileDownload, Status> {
    // Get file info from database
    let mut connection = establish_connection();
    let file = match get_file_by_hash(&mut connection, &file_hash) {
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

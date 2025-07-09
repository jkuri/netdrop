#[macro_use] extern crate rocket;

use rocket::response::content::RawHtml;
use include_dir::{include_dir, Dir};
use rocket::http::ContentType;
use std::path::PathBuf;

static ASSETS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/web/netdrop/dist");

#[get("/<file..>")]
async fn static_files(file: PathBuf) -> Option<(ContentType, Vec<u8>)> {
    let path = file.display().to_string();
    let file_content = ASSETS.get_file(&path)?;
    let content_type = ContentType::from_extension(file.extension()?.to_str()?)?;

    Some((content_type, file_content.contents().to_vec()))
}

#[get("/")]
fn index() -> RawHtml<&'static str> {
    RawHtml(ASSETS.get_file("index.html").map_or("Not found", |f| std::str::from_utf8(f.contents()).unwrap_or("Invalid UTF-8")))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, static_files])
}

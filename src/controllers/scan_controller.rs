use actix_multipart::form::{MultipartForm, tempfile::TempFile};
use actix_web::{get, web, HttpResponse, Responder, post};
use serde_json::json;
use tesseract::ocr;

#[derive(MultipartForm)]
pub struct Upload {
    file: TempFile,
}

#[post("/file/ocr")]
pub async fn upload_file(form: MultipartForm<Upload>) -> HttpResponse {
    const MAX_FILE_SIZE: usize = 1024 * 1024 * 10; // 10 MB

    // reject malformed requests
    match form.file.size {
        0 => return HttpResponse::BadRequest().finish(),
        length if length > MAX_FILE_SIZE => {
            return HttpResponse::BadRequest()
                .body(format!("The uploaded file is too large. Maximum size is {} bytes.", MAX_FILE_SIZE));
        },
        _ => {}
    };
    
    let temp_file_path = form.file.file.path().to_str().unwrap();

    let result = ocr(temp_file_path, "eng");
    match result {
        Ok(data) => {
            println!("{:?}", data);
            HttpResponse::Ok().body(data)
        },
        Err(err) => {
            print!("{:?}", err);
            HttpResponse::BadRequest().body(format!("Error: {}", err))
        },
    }
}

#[get("/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "This is tesseract";

    HttpResponse::Ok().json(json!({"status": "success","message": MESSAGE}))
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(health_checker_handler)
        .service(upload_file);

    conf.service(scope);
}
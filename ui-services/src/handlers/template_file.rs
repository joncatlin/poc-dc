use crate::{db, app_errors::MyError, models::{Language}};
use actix_web::{get, post, web, Error, HttpResponse};
use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use deadpool_postgres::{Client, Pool};
use std::io::Write;

// TODO. Ensure the file permissions are correct
// TODO. Once complete the file should be moved from the directory to its final location with the correct name
#[post("/ui-services/v1/template_files/file-upload")]
async fn upload_template(mut payload: Multipart) -> Result<HttpResponse, Error> {
    debug!("In save_template");
    
    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap();
        let filepath = format!("./template_temp/{}", filename);

        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath))
            .await
            .unwrap();
        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&data).map(|_| f)).await?;
        }
    }

    // create a new chaennel id and return it to the caller
    // upsert_new_channels(
    //     upsert_list: &Vec<models::Channel>,
    //     conn: &PgConnection,

    Ok(HttpResponse::Ok().into())
//    Ok(HttpResponse::Ok().json(results))
}


#[get("/ui-services/v1/template_files/file-upload")]
fn upload_template_html() -> HttpResponse {
    let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body>
            <form target="/ui-services/v1/templates/file-upload" method="post" enctype="multipart/form-data">
                <input type="file" multiple name="file"/>
                <input type="submit" value="Submit"></button>
            </form>
        </body>
    </html>"#;

    HttpResponse::Ok().body(html)
}


#[get("/ui-services/v1/template_files/file-download/{template_id}")]
fn download_template(
    template_id: web::Path<i32>
) -> Result<HttpResponse, Error> {
    use actix_files::NamedFile;
//    use std::path::Path;
    use std::path::PathBuf;

    let template_id = template_id.into_inner();


    // let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    // Ok(NamedFile::open(path)?)
    let path = PathBuf::new(format!("./templates/template_id_{}", template_id));

    if path.exists() {
        return Ok(NamedFile::open(path)?)
    } else {
        // Return 404
    }
}







// fn favicon(req: HttpRequest) -> Result<fs::NamedFile> {
//     Ok(fs::NamedFile::open("./favicon.png")?)
// }

// use actix_files::NamedFile;
// use actix_web::{HttpRequest, Result};
// use std::path::PathBuf;

// async fn index(req: HttpRequest) -> Result<NamedFile> {
//     let path: PathBuf = req.match_info().query("filename").parse().unwrap();
//     Ok(NamedFile::open(path)?)
// }

// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     use actix_web::{web, App, HttpServer};

//     HttpServer::new(|| App::new().route("/{filename:.*}", web::get().to(index)))
//         .bind("127.0.0.1:8088")?
//         .run()
//         .await
// }

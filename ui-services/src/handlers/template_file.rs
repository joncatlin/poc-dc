use crate::{app_errors::MyError, models::{TestTemplate}};
use actix_web::{get, post, web, Error, HttpResponse};
use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use std::io::Write;
use actix_files::NamedFile;
use std::path::PathBuf;
use base64::{encode, decode};

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


// ONLY USED FOR INITAL TESTING. REMOVE WHEN SERVICE IS WORKING
// #[get("/ui-services/v1/template_files/file-upload")]
// fn upload_template_html() -> HttpResponse {
//     let html = r#"<html>
//         <head><title>Upload Test</title></head>
//         <body>
//             <form target="/ui-services/v1/templates/file-upload" method="post" enctype="multipart/form-data">
//                 <input type="file" multiple name="file"/>
//                 <input type="submit" value="Submit"></button>
//             </form>
//         </body>
//     </html>"#;

//     HttpResponse::Ok().body(html)
// }












// #[get("/ui-services/v1/channels/{chan_name}")]
// async fn get_channel_by_name(
//     pool: web::Data<Pool>,
//     name: web::Path<String>,
// ) -> Result<HttpResponse, Error> {








#[get("/ui-services/v1/template_files/file-download/{template_id}")]
async fn download_template(
    template_id: web::Path<i32>
) -> Result<NamedFile, Error> {

    let template_id = template_id.into_inner();

    // Create the path to the file 
    let mut path = PathBuf::new();
    path.push(format!("./templates/template_id_{}.html", template_id));

//    if path.exists() {
        return Ok(NamedFile::open(path)?);
//     } else {
//         // Return 404
// //        return ErrorNotFound();
//    }
}


#[get("/ui-services/v1/docs/dc/{doc_id}")]
async fn download_document(
    doc_id: web::Path<String>
) -> Result<NamedFile, Error> {

    let doc_id = doc_id.into_inner();

    // Create the path to the file 
    let mut path = PathBuf::new();
    path.push(format!("./documents/{}", doc_id));

//    if path.exists() {
        return Ok(NamedFile::open(path)?);
//     } else {
//         // Return 404
// //        return ErrorNotFound();
//    }
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



/// Upload a template inside a JSON structure 
#[post("/ui-services/v1/templates")]
async fn upload (
    temp: web::Json<TestTemplate>,
) -> Result<HttpResponse, Error> {

    info!("Upload request structure is: {:?}", temp);

    // Convert field from base64 before writing it to a file with the template id
    let contents = decode(&temp.document_content).map_err(MyError::DecodeError)?;
    info!("File contents is: {:?}", &contents);
    let mut filepath = PathBuf::new();
    filepath.push(format!("./templates/template_id_{}.html", temp.template_id));
    
    let mut f = web::block(|| std::fs::File::create(filepath)).await.unwrap();
    web::block(move || f.write_all(&contents).map(|_| f)).await?;

    Ok(HttpResponse::Ok().into())
}




/// Download a template inside a JSON structure 
#[get("/ui-services/v1/templates/{temp_id}")]
async fn download (
    temp_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {

    let temp_id = temp_id.into_inner();
    info!("Download request for tempalate_id={}", temp_id);

    // Read the contents of the requested template
    let mut filepath = PathBuf::new();
    filepath.push(format!("./templates/template_id_{}.html", temp_id));
    let file_contents = web::block(|| std::fs::read_to_string(filepath)).await.unwrap();

    // Create the structure to return and encode the file content to Base64, required for Appian
    let result = TestTemplate {
        template_id: temp_id,
        template_name: "".to_string(),
        document_id: "".to_string(),
        document_content: encode(&file_contents),
    };

    Ok(HttpResponse::Ok().json(result))
}





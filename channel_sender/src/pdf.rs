use crate::SEND_TO_VENDOR;

use uuid::Uuid;
//use std::fs;
use std::path::Path;
use serde_json::{Value};
use std::fs::File;
use std::io::prelude::*;
use reqwest::multipart;

// Statics
static TEMP_FILENAME: &'static str = "./index.html";



//************************************************************************
pub async fn send_pdf(account_fields: &Value, pdf_content: String, pdf_service_url: &String, client: &reqwest::Client)
 -> Result<String, Box<dyn std::error::Error>> {

    debug!("In send_pdf. account_fields: {:?}, pdf_content: {:?}, pdf_service_url: {}", account_fields, pdf_content, pdf_service_url);
    let uuid = Uuid::new_v4();

    // // Save the content into a file for the pdf service to use it
    // let temp_filename = "./index.html";
    // let temp_path = Path::new(&temp_filename);
    let temp_path = Path::new(&TEMP_FILENAME);

    let mut file = File::create(&temp_path).expect("Cannot create the temp file for processing the pdf");

    file.write_all(pdf_content.as_bytes()).expect("Failed to write the contents to the temp pdf file");

    file.sync_data().expect("Failed to flush the data to the disk"); // Ensure the data is written so it can be read later by the Reqwest

    // Build request to send to the makepdf service
    info!("Started conversion of pdf");
//    let url = format!("http://docker01:8083/convert/html");
    


// let form = multipart::Form::new()
//     // Adding just a simple text field...
//     .text("username", "seanmonstar")
//     // And a file...
//     .file("photo", "/path/to/photo.png")?;








//    let form = reqwest::blocking::multipart::Form::new()
    let form = multipart::Form::new()
        .text("x", "not used")
        .file("files", "./index.html")
//        .file_name("files", "./index.html")
        .unwrap();

//    let client = reqwest::blocking::Client::new();

    // Only make the call to the vendor solution if env var set correctly. This allows testing volume without making the calls
    if *SEND_TO_VENDOR {

//    if !log_enabled!(Trace) {

        let mut res = client
            .post(pdf_service_url)
            .multipart(form)
            .send();
//            .await
//            .unwrap();
            
        info!("Conversion complete, started file creation");


        

        match res {
            Ok(mut r) => {
//                let filename = format!("./pdf-output/pdf-{}.pdf", uuid);
                let filename = format!("./pdf-output/{}.pdf", account_fields["id"].as_str().unwrap());
                let path = Path::new(&filename);
            
                let mut file = std::fs::File::create(&path)?;
                r.copy_to(&mut file)?;
            },
            Err(e) => {
                error!("Response from send_sms reqwest was failure. Error: {}", e);
            }            
        };
        info!("File creation complete");

    } else {
        return Ok(Uuid::new_v4().to_string())
    }
    // TODO get the message id and return it to the caller
    Ok("".to_string())
}





// fn main() {
//     let path = Path::new("out/lorem_ipsum.txt");
//     let display = path.display();

//     // Open a file in write-only mode, returns `io::Result<File>`
//     let mut file = match File::create(&path) {
//         Err(why) => panic!("couldn't create {}: {}", display, why.description()),
//         Ok(file) => file,
//     };

//     // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
//     match file.write_all(LOREM_IPSUM.as_bytes()) {
//         Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
//         Ok(_) => println!("successfully wrote to {}", display),
//     }
// }
#[macro_use]
extern crate log; 
extern crate env_logger;

//use std::env;
//use futures::StreamExt;
use env_logger::Builder;
use log::LevelFilter;
use chrono::{Local};
use std::io::Write;
//use reqwest::Client;
use uuid::Uuid;

use std::io;
use std::fs::File;
//use std::io::prelude::*;
use std::path::Path;


//************************************************************************
fn main() {

    // Initialize the logger for stdout
    Builder::new()
    .format(|buf, record| {
        writeln!(buf,
            "{} [{}] - {}",
            Local::now().format("%Y-%m-%dT%H:%M:%S"),
            record.level(),
            record.args()
        )
    })
    .filter(None, LevelFilter::Info)
    .init();

    send_makepdf();
}



//************************************************************************
fn send_makepdf() -> Result<(), Box<dyn std::error::Error>> {
    
    // Build request to send to the makepdf service
    info!("Started conversion of pdf");
    let url = format!("http://docker01:8083/convert/html");
    
    let form = reqwest::blocking::multipart::Form::new()
        .text("x", "not used")
        .file("files", "./index.html").unwrap();

    let client = reqwest::blocking::Client::new();

    let resp = client
        .post(&url)
        .multipart(form)
        .send();
    info!("Conversion complete, started file creation");

    match resp {
        Ok(mut r) => {
            println!("success!");
//            let mytext = r.text()?;


            let filename = format!("./output/pdf-{}.pdf", Uuid::new_v4());
            let path = Path::new(&filename);
        
            let mut file = std::fs::File::create(&path)?;
            r.copy_to(&mut file)?;
        
            // match save_in_file(mytext) {
            //     Ok(_) => Ok(()),
            //     Err(e) => {println!("Error {}", e); Ok(())},
            // }
            Ok(())
        },
        Err(e) => Err(e),
    };
    info!("File creation complete");

    Ok(())
}


// fn save_in_file(data: String) -> Result<(), io::Error> {
//     let filename = format!("./output/pdf-{}", Uuid::new_v4());
//     let path = Path::new(&filename);
// //    let display = path.display();

//     let mut file = std::fs::File::create(&path)?;
//     resp.copy_to(&mut file)?;




//     // // Open a file in write-only mode, returns `io::Result<File>`
//     // match File::create(&path) {
//     //     Err(why) => return Err(why),
//     //     Ok(mut file) => {
//     //         file.write_all(data.as_bytes())
//     //     },
//     // };

//     Ok(())
// }
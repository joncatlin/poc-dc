use actix_web::web::Bytes;
use std::str;
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
struct Channel {
    channel_id: i32,
}


static MY_STRING: &'static str = r#"{ "channel_id": 1 }"#;



fn main() {

//    let my_string = MY_STRING.to_string();

    let my_bytes = Bytes::from(r#"{ "channel_id": 1 }"#.as_bytes());
    
//    let my_string = my_bytes.to_vec().as_ptr();

    // let my_string = str::from_utf8(&my_bytes.to_ascii_lowercase()).expect("Failed to convert to utf8");

    // let my_array: Channel = serde_json::from_str(&my_string).expect("Failed to convert to Json");

    println!("Bytes={:?}", my_bytes);


    let my_array: Channel = serde_json::from_str(
        str::from_utf8(&my_bytes.to_ascii_lowercase()).expect("Failed to convert to utf8")
    ).expect("Failed to convert to Json");


    println!("Json is {:?}", my_array);
}




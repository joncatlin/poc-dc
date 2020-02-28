#[macro_use]
extern crate log; 
extern crate env_logger;

use std::env;
use futures::StreamExt;
use env_logger::Builder;
use log::LevelFilter;
use chrono::{Local};
use std::io::Write;

use rdkafka::client::ClientContext;
use rdkafka::config::{ClientConfig};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{CommitMode, Consumer, ConsumerContext, Rebalance};
use rdkafka::error::KafkaResult;
use rdkafka::message::{Headers, Message};
use rdkafka::topic_partition_list::TopicPartitionList;

// Constants
static ACCOUNT_ID: &str = "account_id";
static TOKEN: &str = "token";
static SECRET_PATH: &str = "/run/secrets/";

// Structuer of the expected payload for a digital communication event received via Kafka
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct DC {
    id: String,
    status: String,
    datetime_rfc2822: String,
    event_specific_data: String,

    // Template id - the handle to the template to be used
    // Temaple data - the data required by the template
    // Channel list - a list of channels to send it the template down

}



// A context can be used to change the behavior of producers and consumers by adding callbacks
// that will be executed by librdkafka.
// This particular context sets up custom callbacks to log rebalancing events.
struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {
    fn pre_rebalance(&self, rebalance: &Rebalance) {
        info!("Pre rebalance {:?}", rebalance);
    }

    fn post_rebalance(&self, rebalance: &Rebalance) {
        info!("Post rebalance {:?}", rebalance);
    }

    fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
        info!("Committing offsets: {:?}", result);
    }
}

// A type alias with your custom consumer can be created for convenience.
type LoggingConsumer = StreamConsumer<CustomContext>;


//************************************************************************
async fn consume_and_print() {
    let context = CustomContext;

    // Get the bootstrap servers and topic from the environment variables
    let bootstrap_servers = match env::var("KAFKA_BOOTSTRAP_SERVERS") {
        Ok(val) => val,
        Err(_e) => {
            error!("Could not find environment variable named KAFKA_BOOTSTRAP_SERVERS. Without this variable being set the program will not work.");
            "unconfigured_kafka_bootstrap_servers".to_string()
        }
    };

    let topic = match env::var("KAFKA_TOPIC") {
        Ok(val) => val,
        Err(_e) => {
            error!("Could not find environment variable named KAFKA_TOPIC. Without this variable being set the program will not work.");
            "unconfigured_kafka_topic".to_string()
        }
    };

    let group_id = match env::var("KAFKA_GROUP_ID") {
        Ok(val) => val,
        Err(_e) => {
            error!("Could not find environment variable named KAFKA_GROUP_ID. Without this variable being set the program will not work.");
            "unconfigured_kafka_group_id".to_string()
        }
    };

    let consumer: LoggingConsumer = ClientConfig::new()
        .set("group.id", &*group_id)
        .set("bootstrap.servers", &*bootstrap_servers)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        //.set("statistics.interval.ms", "30000")
        //.set("auto.offset.reset", "smallest")
//        .set_log_level(RDKafkaLogLevel::Debug)
        .create_with_context(context)
        .expect("Consumer creation failed");

    consumer
        .subscribe(&[&topic])
        .expect("Can't subscribe to specified topics");

    // consumer.start() returns a stream. The stream can be used ot chain together expensive steps,
    // such as complex computations on a thread pool or asynchronous IO.
    let mut message_stream = consumer.start();

    while let Some(message) = message_stream.next().await {
        match message {
            Err(e) => warn!("Kafka error: {}", e),
            Ok(m) => {
                let payload = match m.payload_view::<DC>() {
                    None => "",
                    Some(Ok(s)) => s,
                    Some(Err(e)) => {
                        warn!("Error while deserializing message payload: {:?}", e);
                        ""
                    }
                };
                // let payload = match m.payload_view::<str>() {
                //     None => "",
                //     Some(Ok(s)) => s,
                //     Some(Err(e)) => {
                //         warn!("Error while deserializing message payload: {:?}", e);
                //         ""
                //     }
                // };
                info!("key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
                      m.key(), payload, m.topic(), m.partition(), m.offset(), m.timestamp());
                if let Some(headers) = m.headers() {
                    for i in 0..headers.count() {
                        let header = headers.get(i).unwrap();
                        info!("  Header {:#?}: {:?}", header.0, header.1);
                    }
                }
                consumer.commit_message(&m, CommitMode::Async).unwrap();
            }
        };
    }
}


//************************************************************************
#[tokio::main]
async fn main() {

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


    // let (version_n, version_s) = get_rdkafka_version();
    // info!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);

    // let topics = matches.values_of("topics").unwrap().collect::<Vec<&str>>();
    // let brokers = matches.value_of("brokers").unwrap();
    // let group_id = matches.value_of("group-id").unwrap();

    consume_and_print().await
}


//************************************************************************
async fn send_whatsapp() {
    
    let account_id = "ACbd9666b3f5427bb33828653997cb357a".to_string();
    let token = "8cd2cb93c2e4bdcef546690b0868e995".to_string();

    // Send a Whatsapp msg
    let mut params = HashMap::new();
    params.insert("To", "whatsapp:+14805169974");
    params.insert("From", "whatsapp:+14155238886");
    params.insert("Body", "Hello, World! from Rust");
    params.insert("StatusCallback", "http://destini.synology.me:50012/csc/webhooks/whatsapp");
    let url = format!("https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json", account_id);
    let client = Client::new();
    let res = client
        .post(&url)
        .basic_auth(account_id, Some(token))
        .form(&params)
        .send()
        .await
        .unwrap();

    let mytext = res.text().await.unwrap();
    println!("{}", mytext);
}


//************************************************************************
async fn send_sms() {
    let account_id = "ACbd9666b3f5427bb33828653997cb357a".to_string();
    let token = "8cd2cb93c2e4bdcef546690b0868e995".to_string();

    // Send an SMS
    let mut params = HashMap::new();
    params.insert("To", "+14805169974");
    params.insert("From", "+14804053433");
    params.insert("Body", "Hello, World! from Rust");
    params.insert("StatusCallback", "http://destini.synology.me:50012/csc/webhooks/sms");
    let url = format!("https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json", account_id);
    let client = reqwest::Client::new();
    let res = client
        .post(&url)
        .basic_auth(account_id, Some(token))
        .form(&params)
        .send()
        .await
        .unwrap();

    let mytext = res.text().await.unwrap();
    println!("{}", mytext);
}


//************************************************************************
async fn send_email() {
    let account_id = "ACbd9666b3f5427bb33828653997cb357a".to_string();
    let token = "8cd2cb93c2e4bdcef546690b0868e995".to_string();

    // Send an SMS
    let mut params = HashMap::new();
    params.insert("To", "+14805169974");
    params.insert("From", "+14804053433");
    params.insert("Body", "Hello, World! from Rust");
    params.insert("StatusCallback", "http://destini.synology.me:50012/csc/webhooks/sms");
    let url = format!("https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json", account_id);
    let client = reqwest::Client::new();
    let res = client
        .post(&url)
        .basic_auth(account_id, Some(token))
        .form(&params)
        .send()
        .await
        .unwrap();

    let mytext = res.text().await.unwrap();
    println!("{}", mytext);
}


//************************************************************************
fn get_secret(name: &str) -> String {
    let file_name = format!("{}{}", SECRET_PATH, name);
    let secret = match fs::read_to_string(&file_name) {
        Ok(s) => s,
        Err(e) => {
            error!("Error occurred trying to find a secret named:{}. The error was:{}", &file_name, e);
            panic!("secret not found, cannot continue")
        },
    };
    secret
}

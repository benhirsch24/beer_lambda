#[macro_use]
extern crate lambda_runtime as lambda;
#[macro_use]
extern crate serde_derive;
extern crate log;
extern crate rusoto_core;
extern crate rusoto_dynamodb;
extern crate simple_logger;

use lambda::error::HandlerError;
use rusoto_core::{HttpClient, Region};
use rusoto_credential::{EnvironmentProvider};
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ScanInput};
use std::error::Error;

#[derive(Deserialize, Clone)]
struct SlackInput {
    #[serde(rename = "text")]
    text: String,
}

#[derive(Serialize, Clone)]
struct SlackOutput {
    message: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info)?;
    lambda!(my_handler);

    Ok(())
}

fn my_handler(e: SlackInput, _: lambda::Context) -> Result<SlackOutput, HandlerError> {
    let ddb_client = DynamoDbClient::new_with(HttpClient::new().unwrap(), EnvironmentProvider::default(), Region::UsWest2);
    let mut scan_input = ScanInput::default();
    scan_input.table_name = "Beers".to_string();
    match ddb_client.scan(scan_input).sync() {
        Ok(o) => {
            match o.items {
                Some(is) => {
                    for row in is {
                        for (key, val) in &row {
                            if *key != "IsOnTap".to_string() {
                                println!("Key: {} Value: {}", key, val.s.as_ref().unwrap());
                            } else {
                                println!("Key: {} Value: {}", key, val.n.as_ref().unwrap());
                            }
                        }
                    }
                },
                None => println!("No items")
            };
        },
        Err(e) => {
            match e {
                rusoto_dynamodb::ScanError::Unknown(ref resp) => {
                    println!("Unknown error: {:?}", e);
                    println!("Body: {}", std::str::from_utf8(&resp.body).unwrap());
                },
                _ => {
                    dbg!(e);
                }
            }
        }
    };
    Ok(SlackOutput {
        message: format!("Hello, {}!", e.text),
    })
}

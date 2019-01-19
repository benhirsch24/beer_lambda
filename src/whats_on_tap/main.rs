#[macro_use]
extern crate lambda_runtime as lambda;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate rusoto_core;
extern crate rusoto_dynamodb;
extern crate simple_logger;

use lambda::error::HandlerError;
use rusoto_core::{HttpClient, Region};
use rusoto_credential::{DefaultCredentialsProvider};
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ScanInput};
use std::error::Error;

#[derive(Deserialize, Clone)]
struct SlackInput {
}

#[derive(Serialize, Clone)]
struct SlackOutput {
    message: String,
}

impl SlackOutput {
    pub fn with_message(msg: String) -> SlackOutput {
        SlackOutput { message: msg }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info)?;
    lambda!(my_handler);

    Ok(())
}

fn my_handler(_: SlackInput, _: lambda::Context) -> Result<SlackOutput, HandlerError> {
    let ddb_client = DynamoDbClient::new_with(HttpClient::new().unwrap(),
                                              DefaultCredentialsProvider::new().unwrap(),
                                              Region::UsWest2);
    let mut scan_input = ScanInput::default();
    scan_input.table_name = "Beers".to_string();
    let response = match ddb_client.scan(scan_input).sync() {
        Err(e) => {
            error!("Error with dynamodb scan {:?}", e);
            "It looks like there's no beers on tap right now.".to_string()
        },
        Ok(row) => {
            match row.items {
                Some(items) => {
                    let names = items.iter()
                        .map(|r| r.get("Name").as_ref().unwrap().s.as_ref().unwrap().clone())
                        .fold(String::new(), |acc, name| format!("{}\n{}", acc, name));
                    format!("Current beers on tap:{}", names)
                },
                None => "It looks like there's no beers on tap right now.".to_string()
            }
        }
    };
    Ok(SlackOutput::with_message(response))
}

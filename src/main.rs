mod carddb;

use reqwest;
use rusqlite::Connection;

use crate::carddb::{CardCollection, MonsterCardCollection};

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let response = client
        .get("https://db.ygoprodeck.com/api/v7/cardinfo.php?type=Normal%20Monster")
        .send()
        .await
        .unwrap();

    let mcc: carddb::MonsterCardCollection;
    match response.status() {
        reqwest::StatusCode::OK => {
            mcc = match response.json::<MonsterCardCollection>().await {
                Ok(parsed) => parsed,
                Err(e) => panic!("Err {:?}", e),
            }
        }
        _ => { panic!("Uh oh"); }
    }

    let conn = Connection::open("test.db3").unwrap();
    carddb::setup_db(&conn).unwrap();
    mcc.insert_to_db(&conn);
}

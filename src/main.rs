use reqwest;

mod carddb;

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let response = client
        .get("https://db.ygoprodeck.com/api/v7/cardinfo.php?type=Normal%20Monster")
        .send()
        .await
        .unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            match response.json::<carddb::MonsterCardCollection>().await {
                Ok(parsed) => println!("{:?}", parsed),
                Err(e) => println!("Err: {:?}", e),
            }
        }
        _ => { panic!("Uh oh"); }
    }
}

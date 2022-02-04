use reqwest;
use rusqlite::Connection;

mod carddb;

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let response = client
        .get("https://db.ygoprodeck.com/api/v7/cardinfo.php?type=Normal%20Monster")
        .send()
        .await
        .unwrap();

    let coll: carddb::MonsterCardCollection;
    match response.status() {
        reqwest::StatusCode::OK => {
            coll = match response.json::<carddb::MonsterCardCollection>().await {
                Ok(parsed) => parsed,
                Err(e) => panic!("Err {:?}", e),
            }
        }
        _ => { panic!("Uh oh"); }
    }

    // let conn = Connection::open_in_memory().unwrap();
    let conn = Connection::open("test.db3").unwrap();
    carddb::setup_db(&conn);
    coll.insert_to_db(&conn);
    // let mut stmt = conn.prepare("SELECT name, desc, atk, def").unwrap();
    // let m_iter = stmt.query_map([], |row| {
    //     Ok(format!("{}\n{}\natk: {}\tdef: {}\n",
    //             row.get(0).unwrap(),
    //             row.get(1).unwrap(),
    //             row.get(2).unwrap(),
    //             row.get(3).unwrap())
    //     )
    // }).unwrap();
}

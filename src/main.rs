mod carddb;

use reqwest;
use reqwest::Url;
use rusqlite::Connection;

use crate::carddb::{CardCollection, MonsterCardCollection, SpellTrapCardCollection, CardType};

async fn parse_card_data(query: &(CardType, (&str, &str))) {

}

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    const BASE_URL: &str = "https://db.ygoprodeck.com/api/v7/cardinfo.php";
    let queries = vec![
        (CardType::Monster, ("type", "Effect Monster")),
        (CardType::Monster, ("type", "Flip Effect Monster")),
        (CardType::Monster, ("type", "Fusion Monster")),
        (CardType::Monster, ("type", "Gemini Monster")),
        (CardType::Monster, ("type", "Link Monster")),
        (CardType::Monster, ("type", "Normal Monster")),
        (CardType::Monster, ("type", "Normal Tuner Monster")),
        (CardType::Monster, ("type", "Pendulum Effect Monster")),
        (CardType::Monster, ("type", "Pendulum Flip Effect Monster")),
        (CardType::Monster, ("type", "Pendulum Normal Monster")),
        (CardType::Monster, ("type", "Pendulum Tuner Effect Monster")),
        (CardType::Monster, ("type", "Ritual Effect Monster")),
        (CardType::Monster, ("type", "Ritual Monster")),
        (CardType::Monster, ("type", "Spirit Monster")),
        (CardType::Monster, ("type", "Synchro Monster")),
        (CardType::Monster, ("type", "Synchro Pendulum Effect Monster")),
        (CardType::Monster, ("type", "Synchro Tuner Monster")),
        (CardType::Monster, ("type", "Toon Monster")),
        (CardType::Monster, ("type", "Tuner Monster")),
        (CardType::Monster, ("type", "Union Effect Monster")),
        (CardType::Monster, ("type", "XYZ Monster")),
        (CardType::Monster, ("type", "XYZ Pendulum Effect Monster")),
        (CardType::SpellTrap, ("type", "Spell Card")),
        (CardType::SpellTrap, ("type", "Trap Card"))
    ];

    let mut mcc: carddb::MonsterCardCollection;
    let mut stcc: carddb::SpellTrapCardCollection;

    let conn = Connection::open("test.db3").unwrap();
    carddb::setup_db(&conn).unwrap();

    for query in queries {
        let url = Url::parse_with_params(BASE_URL, &[query.1]).unwrap();
        let response = client
            .get(url)
            .send()
            .await
            .unwrap();

        match response.status() {
            reqwest::StatusCode::OK => {
                match query.0 {
                    CardType::Monster => {
                        mcc = match response.json::<MonsterCardCollection>().await {
                            Ok(parsed) => parsed,
                            Err(e) => panic!("Err {:?}", e),
                        };
                        mcc.insert_to_db(&conn);
                    }
                    CardType::SpellTrap => {
                        stcc = match response.json::<SpellTrapCardCollection>().await {
                            Ok(parsed) => parsed,
                            Err(e) => panic!("Err {:?}", e),
                        };
                        stcc.insert_to_db(&conn);
                    }
                }
            }
            _ => { // Insert statuscode handler here.
            }
        }
    }
}

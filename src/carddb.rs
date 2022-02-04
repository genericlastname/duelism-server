use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MonsterCardCollection {
    data: Vec<MonsterCard>,
}

impl MonsterCardCollection {
    pub fn insert_to_db(&self, conn: &Connection) {
        for mc in &self.data {
            conn.execute(
                "INSERT INTO monsters (name, kind, desc, atk, def, level, race)
                VALUES
                    (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![mc.name, mc.kind, mc.desc, mc.atk, mc.def, mc.level, mc.race]
            ).unwrap();
        } 
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpellTrapCardCollection {
    data: Vec<SpellTrapCard>,
}

#[derive(Serialize, Deserialize, Debug)]
struct MonsterCard {
    id: i32,
    name: String,
    #[serde(rename = "type")]
    kind: String,
    desc: String,
    atk: i32,
    def: i32,
    level: i32,
    race: String,
    card_images: Vec<CardImages>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SpellTrapCard {
    id: i32,
    name: String,
    #[serde(rename = "type")]
    kind: String,
    desc: String,
    race: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct CardImages {
    id: i32,
    image_url: String,
    image_url_small: String,
}

pub fn setup_db(conn: &Connection) {
    conn.execute(
        "CREATE TABLE monsters (
            id      INT PRIMARY KEY,
            name    TEXT NOT NULL,
            kind    TEXT,
            desc    TEXT,
            atk     INT,
            def     INT,
            level   INT,
            race    TEXT
        )",
        [],
    ).unwrap();
}

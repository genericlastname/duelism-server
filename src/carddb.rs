use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};

pub enum CardType {
    Monster,
    SpellTrap,
}

pub trait CardCollection {
    fn insert_to_db(&self, conn: &Connection);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MonsterCardCollection {
    data: Vec<MonsterCard>,
}

impl CardCollection for MonsterCardCollection {
    fn insert_to_db(&self, conn: &Connection) {
        for mc in &self.data {
            if mc.card_images.len() > 0 {
                conn.execute(
                    "insert into card_images (id, url, url_small)
                    values
                        (?1, ?2, ?3)",
                        params![mc.card_images[0].id,
                        mc.card_images[0].image_url,
                        mc.card_images[0].image_url_small]
                ).unwrap();
            } else {
                conn.execute(
                    "insert into card_images (id, url, url_small)
                    values
                        (?1, ?2, ?3)",
                        params![-1, "", ""]
                ).unwrap();
            }
            conn.execute(
                "INSERT INTO monsters (name, kind, desc, atk, def, level, race, image_id)
                VALUES
                    (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                params![mc.name,
                        mc.kind,
                        mc.desc,
                        mc.atk,
                        mc.def,
                        mc.level,
                        mc.race,
                        mc.card_images[0].id]
            ).unwrap();
        } 
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpellTrapCardCollection {
    data: Vec<SpellTrapCard>,
}

impl CardCollection for SpellTrapCardCollection {
    fn insert_to_db(&self, conn: &Connection) {
        for stc in &self.data {
            conn.execute(
                "INSERT INTO card_images (id, url, url_small)
                VALUES
                    (?1, ?2, ?3)",
                params![stc.card_images[0].id,
                        stc.card_images[0].image_url,
                        stc.card_images[0].image_url_small]
            ).unwrap();
            conn.execute(
                "INSERT INTO spells_traps (name, kind, desc, race, image_id)
                VALUES
                    (?1, ?2, ?3, ?4, ?5)",
                params![stc.name,
                        stc.kind,
                        stc.desc,
                        stc.race,
                        stc.card_images[0].id]
            ).unwrap();
        } 
    }
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
    card_images: Vec<CardImage>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SpellTrapCard {
    id: i32,
    name: String,
    #[serde(rename = "type")]
    kind: String,
    desc: String,
    race: String,
    card_images: Vec<CardImage>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CardImage {
    id: i32,
    image_url: String,
    image_url_small: String,
}

pub fn setup_db(conn: &Connection) -> rusqlite::Result<()>  {
    conn.execute_batch(
        "create table if not exists card_images (
            id          int primary key,
            url         text,
            url_small   text
        );
        create table if not exists monsters (
            id          int primary key,
            name        text not null,
            kind        text,
            desc        text,
            atk         int,
            def         int,
            level       int,
            race        text,
            image_id    int,
            foreign key(image_id) references card_images(id)
        );
        create table if not exists spells_traps (
            id          int primary key,
            name        text not null,
            kind        text,
            desc        text,
            race        text,
            image_id    int,
            foreign key(image_id) references card_images(id)
        );")?;
    Ok(())
}

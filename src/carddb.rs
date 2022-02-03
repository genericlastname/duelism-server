use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize, Debug)]
// enum Card {
//     MonsterCard(MonsterCard),
//     SpellTrapCard(SpellTrapCard),
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct MonsterCardCollection {
    data: Vec<MonsterCard>,
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


use std::cmp::PartialEq;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Request {
    #[serde(default)]
    update_id: Option<u64>,
    #[serde(default)]
    edited_message: Option<Message>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Message {
    message_id: u64,
    from: Contact,
    chat: Chat,
    date: u64,
    edit_date: u64,
    text: String,
    entities: Vec<Entity>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Contact {
    id: u64,
    is_bot: bool,
    first_name: String,
    last_name: String,
    username: String,
    language_code: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Chat {
    id: i64,
    title: String,
    #[serde(rename="type")]
    type_: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Entity {
    offset: u64,
    length: u64,
    #[serde(rename="type")]
    type_: String,
}

#[cfg(test)]
mod tests {
    extern crate serde_json;

    use super::Request;

    #[test]
    fn it_works() {
        serde_json::from_str::<Request>(r#"{"update_id":309338065,"edited_message":{"message_id":718,"from":{"id":25900594,"is_bot":false,"first_name":"Marco","last_name":"Napetti","username":"Nappa85","language_code":"en-US"},"chat":{"id":-1001249263243,"title":"PoGoRAID Quarto-Casale-Roncade","type":"supergroup"},"date":1519106413,"edit_date":1519126960,"text":"\ud83d\uddc2 Sondaggio disponibilit\u00e0\nMarted\u00ec 20 febbraio 2018\n\nMattina [0]\n\nPrimo pomeriggio [0]\n\nTardo pomeriggio [4]\n\u200e\u251c Marco Napetti\n\u200e\u251c Riccardo\n\u200e\u251c Isabella\n\u200e\u2514 Simone\n\nSera [2]\n\u200e\u251c Marco Napetti\n\u200e\u2514 Isabella\n\n\ud83d\udc65 4 people have voted so far","entities":[{"offset":3,"length":23,"type":"bold"},{"offset":53,"length":7,"type":"bold"},{"offset":66,"length":16,"type":"bold"},{"offset":88,"length":16,"type":"bold"},{"offset":161,"length":4,"type":"bold"}]}}"#).unwrap();
    }
}

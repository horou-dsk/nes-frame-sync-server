use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct GameName {
    pub game: String
}

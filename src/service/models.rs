use mysql::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Account {
    pub id: i32,
    pub email: String,
    pub firebase_id: String
}

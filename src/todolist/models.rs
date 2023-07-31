use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct CreateData {
    pub title: String,
    pub desc: String,
    pub date: i64
}

#[derive(Deserialize, Clone)]
pub struct UpdateData {
    pub title: String,
    pub desc: String,
}


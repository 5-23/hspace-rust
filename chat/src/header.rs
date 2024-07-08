#[derive(serde::Deserialize, Debug)]
pub struct Join {
    pub ip: String,
    pub name: String,
}
#[derive(serde::Deserialize, Debug)]
pub struct Send {
    pub content: String,
}

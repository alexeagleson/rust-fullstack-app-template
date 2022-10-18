use serde::Serialize;

#[derive(Serialize)]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub favourite_food: Option<String>
}
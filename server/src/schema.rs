///Json needed to send message from frontend format to backend and then into database
/// https://serde.rs/ used to serialize data for json format from frontend and backend 
use serde::{Deserialize, Serialize}


// Struct Defining Users Present
/// Struct will be used for call to database from backend to send back to frontend
#[derive(Debug, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
}

/// Strtuct for Users as well created from frontend storeed in database, basically initial from frontend 
/// For Post schema
#[derive(Debug, Deserialize)]
pub struct InitUser {
    pub name: String, 
}


/// Message stored in database from backend call to send back to frontend 
/// Get Schema
#[derive(Debug, Serialize)]
pub struct Message {
    pub user_id: i32,
    pub username: String, 
    pub content: String,
    pub created_at: String,
}


///Respresents Json message from frontend to put/post into backend
#[derive(Debug, Deserialize)]
pub struct InitMessage {
    pub user_id: i32, 
    pub username: String, 
    pub content: String, 
    pub created_at: String, 
}
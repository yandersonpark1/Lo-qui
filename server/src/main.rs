use std::error::Error;
use sqlx::Row;
use sqlx::postgres;

use std::collections::HashSet;
use rand::Rng;

use axum::{routing::{get, post},  Router};

use sqlx::postgres;
mod get;

type SimpleResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct Message {
    pub user: User.unique_id
    pub username: User.name
    pub content: String,
    pub created_at: String,
}

// Struct Defining Users Present
#[derive(Debug)]
struct User {
    unique_id: i32,
    pub name: String,
}

/// struct for connection state for api methods
#[derive(Debug)]
struct ConnectionState {
    pub db: PgPoolOptions,  
}

//Active Users Defined as a Hashset
static mut activeusers: HashSet<i32> = HashSet::<i32>;

impl user {
    pub fn new(username: String) -> SimpleResult<Self> {
        let mut rng = rand::rng();
        let mut userid: i32 = rng.random();

        //If the randomly selected ID is already taken, rerandomize
        while activeusers.contains(userid) {
            userid = rng.random();
        }

        //Add to list of users present
        activeusers.insert(userid);

        //Return self
        Ok(Self {
            unique_id: userid;
            name: username;
        })
    }

    //Return self name
    pub fn get_name() -> SimpleResult<String> {
        Ok(Self.name)
    }

    //Return self ID
    pub fn get_id() -> SimpleResult<i32> {
        Ok(Self.unique_id)
    }
}


/// taken from https://github.com/launchbadge/sqlx
/// sqlx is used to manager our database 
/// axum is working for our https method communicating between the backend and the database
/// connects to database and creates our router
#[tokio::main]
async fn main() -> Result<(), sqlx::Error>> {
    let url = env::var("DATABASE_URL").unwrap();
    
    let pool = sqlx::postgres::PgPoolOptions::new().connect(&url).context("Unable to connect to the database")?;
    
    sqlx::migrate!("./migrations").run(&pool).await?;

    ///creating router for Axum for chatroom HTTP methods; https://docs.rs/axum/latest/axum/routing/struct.Router.html
    let app = Router::new().route().with_state(&pool)
        .route("/health", get(health_check))
        /// may only need this route for post and get 
        .route("/messages", post(message).get(message))
        .route("/users", get(list_users))
        .route("/users/{id}", get(get_user))
        // .route("/messages", get(move |mid| message_history(mid)))
        .with_state(&pool);

    let state = ConnectionState{db: &pool}


    Ok(())
}



#[cfg(test)]
mod test {

    #[test]
    fn create_user() {
        let username = "UserA".to_string();
        let user = username::new(username);
        assert_eq!(
            user.get_name(), "UserA".to_string()
        );
    }

    #[test]
    fn hashset_push() {
        let username = "UserA".to_string();
        let user = username::new(username);
        assert!(
            activeusers.contains(user)
        );
    }
}
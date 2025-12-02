use std::error::Error;
use sqlx::Row;
use std::collections::HashSet;
use rand::Rng;
use axum::{routing::{get, post},  Router};

mod get;

type SimpleResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
struct Message {
    usernam: user,
    content: String,
    created_at: String,
}

// Struct Defining Users Present
#[derive(Debug)]
struct user {
    unique_id: i32,
    pub name: String,
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

fn post_method(content:String, ) -> Result<(), Box<dyn Error>> {

}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = env::var("DATABASE_URL").unwrap();
    let pool = sqlx::postgres::PgPool::connect(url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    ///creating router for Axum for chatroom HTTP methods; https://docs.rs/axum/latest/axum/routing/struct.Router.html
    let app = router::<()>::new().route(); 


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
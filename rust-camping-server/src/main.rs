use chrono::Datelike;
use dotenv;
use mongodb::{bson::doc, options::ClientOptions};
use mongodb::bson::Document;
use trooptrack_rust::apis::{Error, events_api, tokens_api};
use trooptrack_rust::apis::configuration::Configuration;
use trooptrack_rust::apis::events_api::GetV1EventsError;
use trooptrack_rust::apis::tokens_api::PostV1TokensError;
use trooptrack_rust::models::events_list_entity::EventsListEntity;
use trooptrack_rust::models::token_users_response::TokenUsersResponse;

use state::State;

mod state;
mod user;
mod routes;

fn cfg() -> Configuration {
    Configuration {
        base_path: "https://shakertroop15.trooptrack.com/api".to_owned(),
        ..Default::default()
    }
}

#[tokio::main]
async fn main() -> tide::Result<()> {
    femme::start();

    let db_uri = dotenv::var("MONGO_URI")
        .unwrap_or(String::from("mongodb://localhost:27017/"));
    let listen_addr = dotenv::var("LISTEN_ADDR")
        .unwrap_or(String::from("localhost:9080"));

    let state = State::new(&db_uri).await?;
    let mut app = tide::with_state(state);
    //
    // app.with(tide::log::LogMiddleware::new());
    //
    // app.with(tide::sessions::SessionMiddleware::new(
    //     tide::sessions::MemoryStore::new(),
    //     dotenv::var("TIDE_SECRET")
    //         .expect(
    //             "Please provide a TIDE_SECRET value of at \
    //                   least 32 bytes in order to run this example",
    //         )
    //         .as_bytes(),
    // ));
    //
    // app.with(tide::utils::Before(
    //     |mut request: tide::Request<state::State>| async move {
    //         let session = request.session_mut();
    //         let visits: usize = session.get("visits").unwrap_or_default();
    //         session.insert("visits", visits + 1).unwrap();
    //         request
    //     },
    // ));
    app.at("/").get(routes::get_root);
    app.at("/v1/list").get(routes::list_dbs);
    app.at(&format!("/v1/{}", "/:db/list")).get(routes::list_colls);
    // app.at(&format!("/v1/{}", "/:db/:collection")).post(routes::insert_doc);
    // app.at(&format!("/v1/{}", "/:db/:collection")).get(routes::find_doc);
    // app.at(&format!("/v1/{}", "/:db/:collection/update")).get(routes::update_doc);
    app.listen(&listen_addr).await?;

    Ok(())
}

async fn mongo_test() -> mongodb::error::Result<()> {
    let uri = dotenv::var("MONGO_URI").expect("MONGO_URI expected");
    let mut client_options = ClientOptions::parse_async(uri).await?;
    // only set credential if password is set
    if let Ok(pass) = dotenv::var("MONGO_PASSWORD") {
        let credential = mongodb::options::Credential::builder()
            .username(dotenv::var("MONGO_USER").unwrap_or("MONGO_USER".to_string()))
            .password(pass)
            .build();
        client_options.credential = Some(credential);
    }
    let client = mongodb::Client::with_options(client_options)?;

    println!("Databases: ");
    for x in client.list_databases(None, None).await? {
        println!("  Database: {:?}", x);
    }

    let db_name = dotenv::var("MONGO_DATABASE").unwrap_or("troop15".to_string());
    let db = client.database(&db_name);
    db.run_command(doc! { "ping": 1 }, None).await?;
    println!("Pinged your deployment. You successfully connected to MongoDB!");

    let coll: mongodb::Collection<Document> = db.collection("users");
    let cnt = coll.count_documents(None, None).await?;
    println!("Users collection count: {}", cnt);
    let u = coll.find_one(doc! { "age": 25}, None).await.expect("Failed to find user");
    match u {
        Some(user) => {
            println!("User: {:?}", user);
        }
        None => {
            println!("User not found");
        }
    }
    Ok(())
}

async fn ttapi_main() {
    let config = cfg();
    dotenv::dotenv().expect("Failed to read .env file");
    let default_int = 1;

    match dotenv::var("TT_TOKEN") {
        Ok(tok) => {
            println!("Token: {:?}", tok);
            match get_some_events(&config).await {
                Ok(res) => {
                    for event in res.events.unwrap_or(vec![]) {
                        let dt = chrono::DateTime::parse_from_rfc3339(&event.activity_at.unwrap_or("".to_string()))
                            .expect("Failed to parse date");

                        println!("Event: id={}, mo={}/{}/{}, titre={}",
                                 event.event_id.as_ref().unwrap_or(&default_int),
                                 dt.month(), dt.day(), dt.year(),
                                 event.title.as_ref().unwrap_or(&"No title".to_string()),
                        );
                    }
                }
                Err(err) => {
                    println!("Failed to get events: {:?}", err);
                }
            }
        }
        Err(_) => {
            println!("TT_TOKEN not found in .env file, attempting a new login");
            match login(&config).await {
                Ok(res) => {
                    dbg!("Login successful: {:?}", &res);
                    let tok = &res.users.expect("users array should be present")
                        .get(0)
                        .expect("users array should not be empty")
                        .token
                        .as_ref()
                        .expect("token should be present")
                        .clone()
                        ;
                    println!("Token: {:?}", tok);
                }
                Err(err) => {
                    println!("Login failed: {:?}", err);
                }
            }
        }
    }
}


async fn login(config: &Configuration) -> Result<TokenUsersResponse, Error<PostV1TokensError>> {
    dotenv::dotenv().expect("Failed to read .env file");
    let x_partner_token = dotenv::var("TT_PARTNER_TOKEN").expect("TT_PARTNER_TOKEN expected");
    let x_username = dotenv::var("TT_USER_ID").expect("TT_USER_ID expected");
    let x_user_password = dotenv::var("TT_PASSWORD").expect("TT_PASSWORD expected");

    tokens_api::post_v1_tokens(
        &config,
        &x_partner_token,
        &x_username,
        &x_user_password,
    ).await
}

async fn get_some_events(config: &Configuration) -> Result<EventsListEntity, Error<GetV1EventsError>> {
    let x_partner_token = dotenv::var("TT_PARTNER_TOKEN").expect("TT_PARTNER_TOKEN expected");
    let x_user_token = dotenv::var("TT_TOKEN").expect("TT_TOKEN expected");
    events_api::get_v1_events(
        &config,
        &x_partner_token,
        &x_user_token,
        "2024-01-01",
        "2024-12-31",
    ).await
}

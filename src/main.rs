/*
 * Routes required:
 * /init        (post)  Initialize a player connection with the server
 * /question    (get)   Return the current question details the question's id
 * /answer:id   (post)  Answer question with id `id`
 * /leaderboard (get)   Get the current leaderboard
 *
 * /next_round  (post)  Start the next round [privileged]
 * /pause_round (post)  Pause the current round [privileged]
 */

use axum::{
    extract::Extension,
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

/*
 * Store the current quiz session
 * - Which question set is being used
 * - Which question is currently being served
 * - Addresses of all the users connected
 */
#[derive(Serialize, Deserialize)]
struct State {
    counter: u64,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = Arc::new(Mutex::new(State {
        counter: 0,
    }));

    let app = Router::new()
        .route("/", get(root))
        .layer(Extension(state.clone()));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root(Extension(state): Extension<Arc<Mutex<State>>>) -> (StatusCode, Json<Temp>) {

    let mut state = state.lock().unwrap();
    state.counter += 1;

    (StatusCode::OK, Json(Temp {
        a: "A".to_string(),
        b: state.counter,
    }))
}


#[derive(Serialize)]
struct Temp {
    a: String,
    b: u64,
}

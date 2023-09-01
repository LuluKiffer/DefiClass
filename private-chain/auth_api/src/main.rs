use std::str::FromStr;

use axum::{Json, Router};
use axum::routing::{get, post};
use axum::body::{boxed, Body};
use axum::http::{Response, StatusCode};
use secp256k1::SecretKey;
use serde::Deserialize;
use tower::ServiceExt;
use tower_http::services::ServeDir;
use web3::types::{Address, U256, TransactionParameters};
use web3::Web3;

#[derive(Deserialize)]
struct CreateUserPayload {
    email: String,
    eth_address: String,
}

#[tokio::main]
async fn main() {
    let db = sled::open("./user_storage").expect("failed to open DB");
    let app = Router::new().route("/auth", post({
        let tree_e2a = db.open_tree("e2a").expect("failed to open tree in DB");
        let tree_a2e = db.open_tree("a2e").expect("failed to open tree in DB");
        move |body| new_user(body, tree_e2a, tree_a2e)
    })).fallback_service(get(|req| async move {
        match ServeDir::new("../explorer/app").oneshot(req).await {
            Ok(res) => res.map(boxed),
            Err(err) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(boxed(Body::from(format!("error: {err}"))))
                .expect("error response"),
        }
    }));

    axum::Server::bind(&"0.0.0.0:9000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn new_user(Json(payload): Json<CreateUserPayload>, db_e2a: sled::Tree, db_a2e: sled::Tree) -> (StatusCode, String) {
    if db_e2a.contains_key(&payload.email).expect("failed to read DB") {
        (
            StatusCode::BAD_REQUEST,
            format!("Email already registered: {}", payload.email),
        )
    } else if db_a2e.contains_key(&payload.eth_address).expect("failed to read DB") {
        (
            StatusCode::BAD_REQUEST,
            format!("Ethereum address already registered: {}", payload.eth_address),
        )
    } else {
        // create faucet key object for signing
        let faucet_sk = SecretKey::from_str("bed3f59fdad0700aa9d56fde0ebdc6be5bf48854119c20a2d1075a9222b568ba").unwrap();

        // prepare tx giving ETH starting balance to the new user
        let http = web3::transports::Http::new("http://localhost:8545").unwrap();
        let web3 = Web3::new(http);
        let res = Address::from_str(&payload.eth_address);
        if res.is_err() {
            return (StatusCode::BAD_REQUEST, "Invalid Ethereum address.".to_owned());
        }
        let to = res.unwrap();
        let tx_object = TransactionParameters {
            to: Some(to),
            value: U256::exp10(18), // 1 ETH
            gas_price: Some(U256::exp10(12)), // 1000 GWei
            max_fee_per_gas: Some(U256::exp10(12)), // 1000 Gwei
            max_priority_fee_per_gas: Some(U256::exp10(12)), // 1000 Gwei
            ..Default::default()
        };
        let signed_tx = web3.accounts().sign_transaction(tx_object, &faucet_sk).await.expect("failed to sign tx");

        // print raw tx hex data
        /*for byte in &signed_tx.raw_transaction.0 {
            print!("{:02x}", byte);
        }
        println!("");*/

        // submit tx to node
        if let Err(err) = web3.eth().send_raw_transaction(signed_tx.raw_transaction).await {
            return (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", err));
        }

        // insert the new user into the DBs
        let email: &str = &payload.email;
        let eth_addr: &str = &payload.eth_address;
        db_e2a.insert(email, eth_addr).expect("failed to write DB");
        db_a2e.insert(eth_addr, email).expect("failed to write DB");
        (
            StatusCode::OK,
            "Success!".to_owned(),
        )
    }
}

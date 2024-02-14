use std::sync::Mutex;

use serde::{Deserialize, Serialize};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

/* CONSTANTS */
const QUARTERS_INDEX: usize = 0;
const DIMES_INDEX: usize = 1;
const NICKELS_INDEX: usize = 2;
const PENNIES_INDEX: usize = 3;

#[derive(Serialize, Debug)]
struct Denomination {
    name: String,
    value: f32,
}

#[derive(Serialize, Debug)]
struct Reserve {
    denomination: Denomination,
    current_count: u8,
    max_count: u8,
}

#[derive(Deserialize)]
struct QueryData {
    deno_name: String, // The name of the denomination to modify
    count: String,     // The count of coins to modify
}

#[derive(Deserialize)]
struct CoinRequestBody {
    denomination: String,
    count: u8,
}

#[derive(Serialize)]
struct APIResponse {
    message: Option<String>,
}

enum APIError {
    AppStateLockBusy,
    IllegalDenominationName,
}

struct AppState {
    app_name: String,
    coin_bank_reserves: Mutex<Vec<Reserve>>,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello {app_name}!")
}

fn get_coin_index(denomination: String) -> Option<usize> {
    match denomination.to_lowercase().as_str() {
        "quarter" => Some(QUARTERS_INDEX),
        "dime" => Some(DIMES_INDEX),
        "nickel" => Some(NICKELS_INDEX),
        "penny" => Some(PENNIES_INDEX),
        _ => None,
    }
}

#[post("/add_coin")]
async fn add_coin(
    request_body: web::Json<CoinRequestBody>,
    data: web::Data<AppState>,
) -> impl Responder {
    let denomination = &request_body.denomination;
    let coin_index = match get_coin_index(denomination.to_string()) {
        Some(coin_index) => coin_index,
        None => return HttpResponse::NotAcceptable().json(APIResponse {
            message: Some(format!("denomination name {} is not acceptable.  Acceptable values are 'quarter', 'dime', 'nickel' and 'penny'", request_body.denomination))})
    };

    // Update the global mutable state with the new count of coins
    let mut reserve_to_update = match data.coin_bank_reserves.lock() {
        Ok(reserves) => reserves,
        Err(_) => {
            return HttpResponse::Locked().json(APIResponse {
                message: Some("Application state data is locked".to_string()),
            })
        }
    };

    reserve_to_update[coin_index].current_count += request_body.count;

    HttpResponse::Ok().json(&reserve_to_update[coin_index])
}

// async fn total(data: web::Data<AppState>) -> impl Responder {
//     let reserves = &data.coin_bank_reserves;
//     let current_total: u8 = reserves
//         .iter()
//         .map(|reserve| match reserve.denomination.name.as_str() {
//             "Quarters" => 25 * reserve.current_count,
//             "Dimes" => 10 * reserve.current_count,
//             "Nickels" => 5 * reserve.current_count,
//             "Pennies" => 1 * reserve.current_count,
//             _ => 0,
//         })
//         .sum();

//     format!("{}", current_total as f32 * 0.01)
// }

// #[get("/reserves")]
// async fn get_reserves(data: web::Data<AppState>) -> impl Responder {
//     let reserves = data.coin_bank_reserves.lock().unwrap();

//     let return_reserves = reserves;

//     return_reserves
// }

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let reserves = vec![
        Reserve {
            denomination: Denomination {
                name: String::from("Quarters"),
                value: 0.25,
            },
            current_count: 4,
            max_count: 20,
        },
        Reserve {
            denomination: Denomination {
                name: String::from("Dimes"),
                value: 0.10,
            },
            current_count: 8,
            max_count: 50,
        },
        Reserve {
            denomination: Denomination {
                name: String::from("Nickels"),
                value: 0.05,
            },
            current_count: 0,
            max_count: 15,
        },
        Reserve {
            denomination: Denomination {
                name: String::from("Pennies"),
                value: 0.01,
            },
            current_count: 0,
            max_count: 50,
        },
    ];

    let app_state = web::Data::new(AppState {
        app_name: String::from("Coin Changer Server"),
        coin_bank_reserves: Mutex::new(reserves),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(index)
            // .service(get_reserves)
            .route("/hey", web::get().to(manual_hello))
            .service(add_coin)
        // .route("/total", web::get().to(total))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

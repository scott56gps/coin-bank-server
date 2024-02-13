use serde::Serialize;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[derive(Serialize)]
struct Denomination {
    name: String,
    value: f32,
}

#[derive(Serialize)]
struct Reserve {
    denomination: Denomination,
    current_count: u8,
    max_count: u8,
}

struct AppState {
    app_name: String,
    coin_bank_reserves: [Reserve; 4],
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello {app_name}!")
}

async fn total(data: web::Data<AppState>) -> impl Responder {
    let reserves = &data.coin_bank_reserves;
    let current_total: u8 = reserves
        .iter()
        .map(|reserve| match reserve.denomination.name.as_str() {
            "Quarters" => 25 * reserve.current_count,
            "Dimes" => 10 * reserve.current_count,
            "Nickels" => 5 * reserve.current_count,
            "Pennies" => 1 * reserve.current_count,
            _ => 0,
        })
        .sum();

    format!("{}", current_total as f32 * 0.01)
}

#[get("/reserves")]
async fn get_reserves(data: web::Data<AppState>) -> [Reserve; 4] {
    let reserves = data.coin_bank_reserves;

    let return_reserves = reserves;

    return_reserves
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let reserves = [
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
                value: 0.05
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
        coin_bank_reserves: reserves,
    });

    HttpServer::new(|| {
        App::new()
            .app_data(app_state)
            .service(index)
            .service(get_reserves)
            .route("/hey", web::get().to(manual_hello))
            .route("/total", web::get().to(total))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

use axum::{Json, Router};
use axum::routing::get;
use tokio::time::Instant;

fn fib (n: i32) -> i32 {
    if n <= 0 {
        0
    } else if n== 1{
        1
    } else {
        fib (n-1)  + fib(n-2)
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(get_fib));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

const FIB_AMOUNT: usize = 32;
async fn get_fib() -> Json<Vec<i32>> {
    let now = Instant::now();
    let mut array = vec![0; FIB_AMOUNT];
    for int in 0..FIB_AMOUNT {
        array[int] = fib(int as i32);
    }
    Json(array)
}
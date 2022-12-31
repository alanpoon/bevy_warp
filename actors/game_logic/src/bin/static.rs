#[tokio::main(flavor = "current_thread")]
async fn main() {
    warp::serve(warp::fs::dir("www/public"))
    .run(([127, 0, 0, 1], 3030))
    .await;
}
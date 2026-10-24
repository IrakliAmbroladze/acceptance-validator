fn main() {
    println!("Hello, world!");
}

async fn health() -> &'static str {
    "ok"
}

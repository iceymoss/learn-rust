use learn_rust::file::write::write;
use learn_rust::file::read::read;

#[tokio::main]
async fn main() {
    println!("Welcome to Ubuntu!");

    write().await;
    read().await;
}
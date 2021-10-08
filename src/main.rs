mod service;
mod dao;

use service::http_client::HttpClient;
use service::tokio_rt::drop_tokio_rt;


fn get_from(url: &str) -> String {
    let client = HttpClient::new();
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    client.get_text(url, move |r| {
        if let Ok(text) = r {
            tx.send(text);
        }
    });

    rx.blocking_recv().unwrap_or_else(|| "## ERROR REQUEST ##".to_string())
}

fn main() {
    println!("Hello, world!");

    let text = get_from("https://docs.rs/reqwest/");
    println!("GET Response: {}", text);

    drop_tokio_rt();
}

use reqwest::{Client, Error, RequestBuilder};

use super::tokio_rt::get_tokio_rt;


#[derive(Clone)]
pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    pub fn new() -> Self {
        Self {
            client: Client::new()
        }
    }

    // async fn send_request(req: &RequestBuilder) -> Result<>

    pub fn get_text<T>(&self, url: &str, handle: T)
        where T: FnOnce(Result<String, Error>) + Send + 'static {
        let req = self.client.get(url);
        get_tokio_rt().spawn(async move {
            let text = (async {
                req.send().await?.text().await
            }).await;
            handle(text);
        });
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}

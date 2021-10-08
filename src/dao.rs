use crate::service::http_client::HttpClient;


#[derive(Clone, Default)]
pub struct UrlContent {
    pub url: String,
    pub content: String,
    pub status_code: i32,
}

#[derive(Clone, Default)]
pub struct UrlDataAccessor {
    client: HttpClient,
}

impl UrlDataAccessor {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn load_url<U, T>(&self, url: U, handle: T)
        where U: AsRef<str>,
              T: FnOnce(UrlContent) + Send + 'static {
        let url = url.as_ref();
        let own_url = url.to_string();
        self.client.get_text(url, move |r| {
            let content = match r {
                Ok(text) => UrlContent {
                    url: own_url,
                    content: text,
                    status_code: 0,
                },
                Err(e) => UrlContent {
                    url: e.url().map_or(own_url, |u| u.to_string()),
                    content: Default::default(),
                    status_code: e.status().map_or(-1, |c| c.as_u16() as i32),
                }
            };

            handle(content);
        })
    }
}

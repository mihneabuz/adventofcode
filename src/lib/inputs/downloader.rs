use std::{io, thread, time};

use reqwest::blocking::{Client, ClientBuilder};

use crate::util::into_io_error;

pub struct Downloader {
    client: Client,
    cookie: String,
    timeout: time::Duration,
    last_download: time::Instant,
}

impl Downloader {
    pub fn new(agent: String, cookie: String) -> Self {
        Self::with_timeout(agent, cookie, time::Duration::from_secs(5))
    }

    pub fn with_timeout(agent: String, cookie: String, timeout: time::Duration) -> Self {
        Self {
            client: ClientBuilder::new().user_agent(agent).build().unwrap(),
            cookie,
            timeout,
            last_download: time::Instant::now(),
        }
    }

    pub fn get(&mut self, url: &str) -> io::Result<String> {
        while self.last_download + self.timeout > time::Instant::now() {
            thread::yield_now();
        }

        let text = self
            .client
            .get(url)
            .header("Cookie", &self.cookie)
            .send()
            .map_err(into_io_error)?
            .text()
            .map_err(into_io_error)?;

        self.last_download = time::Instant::now();

        Ok(text)
    }
}

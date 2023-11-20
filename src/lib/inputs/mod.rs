mod cache;
mod downloader;

use std::io;

use crate::{challenge::ChallengeObject, util::into_io_error};

use cache::FileCache;
use downloader::Downloader;

pub struct AocInputs {
    cache: FileCache,
    downloader: Option<Downloader>,
}

struct AocCredentials {
    user_agent: String,
    session_token: String,
}

impl AocCredentials {
    fn read(config: &str) -> io::Result<Self> {
        let config = config::Config::builder()
            .add_source(config::File::new(config, config::FileFormat::Yaml))
            .build()
            .map_err(into_io_error)?;

        Ok(Self {
            user_agent: config.get("user_agent").map_err(into_io_error)?,
            session_token: config.get("session_token").map_err(into_io_error)?,
        })
    }
}

impl AocInputs {
    pub fn new(cache_root: String, downloader: Option<String>) -> io::Result<Self> {
        let downloader = match downloader {
            Some(config) => {
                let creds = AocCredentials::read(&config)?;
                Some(Downloader::new(
                    creds.user_agent,
                    format!("session={}", creds.session_token),
                ))
            }

            None => None,
        };

        Ok(Self {
            cache: FileCache::open(cache_root)?,
            downloader,
        })
    }

    pub fn get_inputs(&mut self, challenges: &mut [ChallengeObject]) -> io::Result<()> {
        for challenge in challenges {
            challenge.input = self.input(challenge)?;
        }

        Ok(())
    }

    pub fn input(&mut self, challenge: &ChallengeObject) -> io::Result<String> {
        let key = format!("{}_day{}", challenge.year, challenge.day);
        if let Ok(input) = self.cache.get(&key) {
            return Ok(input);
        }

        if let Some(downloader) = self.downloader.as_mut() {
            println!(
                "Downloading input for {} day {}...",
                challenge.year, challenge.day
            );

            let url = format!(
                "https://adventofcode.com/{}/day/{}/input",
                challenge.year, challenge.day
            );

            let input = downloader.get(&url)?;

            self.cache.set(key, input.trim())?;

            return Ok(input.trim().to_string());
        }

        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Input for {} day {}", challenge.year, challenge.day),
        ))
    }
}

#![allow(dead_code)]
use anyhow::Result;
use ip2location::DB;
use std::{path::PathBuf, sync::Arc};
use tokio::{fs, sync::Mutex};
use tracing::{info, warn};

#[derive(Debug, Clone)]
pub struct IpFinder {
    db: Arc<Mutex<DB>>,
}

impl IpFinder {
    pub fn new() -> Self {
        IpFinder {
            db: Arc::new(Mutex::new(
                DB::from_file("./IP2LOCATION-LITE-DB9.BIN").unwrap(),
            )),
        }
    }

    pub async fn find_country(self, ip: &str) -> Option<String> {
        if let Ok(location) = self.db.clone().lock().await.ip_lookup(ip) {
            return Some(location.country.unwrap().long_name);
        } else {
            Some(String::from("Unknown"))
        }
    }

    pub async fn find_country_short(self, ip: &str) -> Option<String> {
        if let Ok(location) = self.db.clone().lock().await.ip_lookup(ip) {
            return Some(location.country.unwrap().short_name);
        } else {
            Some(String::from("US"))
        }
    }

    pub async fn find_city(self, ip: &str) -> Option<String> {
        if let Ok(location) = self.db.clone().lock().await.ip_lookup(ip) {
            return Some(location.city.unwrap());
        } else {
            Some(String::from("Unknown"))
        }
    }
}

/// ### Image Store
/// stores and reads images in a folder
#[derive(Debug, Clone)]
pub struct ImageStore {
    pub path: PathBuf,
}

impl ImageStore {
    pub fn new(path: PathBuf) -> Self {
        ImageStore { path }
    }

    pub async fn store_image(
        &self,
        name: &str,
        image: Vec<u8>,
        country_short: Option<&str>,
    ) -> Result<()> {
        if let Some(country_short) = country_short {
            let path = self.path.join(format!("{}-{}", country_short, name));
            fs::write(path, image).await?;
        } else {
            let path = self.path.join(name);
            fs::write(path, image).await?;
        }
        Ok(())
    }

    pub async fn read_image(&self, name: &str, country_short: Option<String>) -> Result<Vec<u8>> {
        let path = self.path.join(name).join(name);
        if path.exists() {
            if let Some(country_short) = country_short {
                let path = self
                    .path
                    .join(name)
                    .join(format!("{}-{}", country_short, name));
                info!("Country path: {:?}", path);
                if path.exists() {
                    return Ok(fs::read(path).await?);
                }
            } else {
                info!("path: {:?}", path);
                return Ok(fs::read(path).await?);
            }
        } else {
            warn!("Image not found: {:?}, Country: {:?}", name, country_short);
        }

        Err(anyhow::anyhow!("Image not found"))
    }
}

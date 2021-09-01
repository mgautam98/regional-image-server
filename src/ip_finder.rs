use std::path::PathBuf;
use anyhow::Result;
use tokio::fs;
use ip2location::DB;

pub struct IpFinder {
    db: DB,
}

impl IpFinder {
    pub fn new() -> Self {
        IpFinder {
            db: DB::from_file("./IP2LOCATION-LITE-DB9.BIN").unwrap(),
        }
    }

    pub fn find_country(mut self, ip: &str) -> Option<String> {
        let location = self.db.ip_lookup(ip).unwrap();
        Some(location.country.unwrap().long_name)
    }

    pub fn find_country_short(mut self, ip: &str) -> Option<String> {
        let location = self.db.ip_lookup(ip).unwrap();
        Some(location.country.unwrap().short_name)
    }

    pub fn find_city(mut self, ip: &str) -> Option<String> {
        let location = self.db.ip_lookup(ip).unwrap();
        Some(location.city.unwrap())
    }
}

/// ### Image Store
/// stores and reads images in a folder
#[derive(Debug)]
pub struct ImageStore {
    pub path: PathBuf,
}

impl ImageStore {
    pub fn new(path: PathBuf) -> Self {
        ImageStore { path }
    }

    pub async fn store_image(&self, name: &str, image: Vec<u8>, country_short: Option<String>) -> Result<()> {
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
        let path = self.path.join(name);

        if path.exists() {
            if let Some(country_short) = country_short {
                let path = self.path.join(format!("{}-{}", country_short, name));
                if path.exists() {
                    return Ok(fs::read(path).await?);
                }
            } else {
                return Ok(fs::read(path).await?);
            }
        }

        Err(anyhow::anyhow!("Image not found"))
    }
}
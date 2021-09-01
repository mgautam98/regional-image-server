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

/*
let make_svc = make_service_fn(move |conn: &AddrStream| {
        let addr = conn.remote_addr();
        async move {
            let addr = addr.clone();
            Ok::<_, Infallible>(service_fn(move |req| hello(req, addr.clone())))
        }
    });
*/

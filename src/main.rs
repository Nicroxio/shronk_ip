use rand::prelude::*;

#[macro_use]
extern crate rocket;
use maxminddb;
use rocket::response::content::RawHtml;
use rocket::{catch, catchers};
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::{Header, Status},
    request::{FromRequest, Outcome, Request},
    serde::json::Json,
    Response,
};

use serde;
use std::net::{IpAddr, Ipv4Addr};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(crate = "rocket::serde")]
struct CityData {
    ip: String,
    city: String,
    country: String,
    is_in_european_union: bool,
    iso_code: String,
}

struct RealIp<'r>(&'r str);

#[derive(Debug)]
enum RealIpError {
    MissingHeader,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RealIp<'r> {
    type Error = RealIpError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.headers().get_one("X-Forwarded-For") {
            Some(ip) => Outcome::Success(RealIp(ip)),
            None => Outcome::Error((Status::BadRequest, RealIpError::MissingHeader)),
        }
    }
}

#[catch(400)]
fn bad_request(req: &Request) -> RawHtml<String> {
    let error_message = if let Some(error) = req.local_cache(|| None::<RealIpError>) {
        match error {
            RealIpError::MissingHeader => "Missing X-Forwarded-For header".to_string(),
        }
    } else {
        "Bad request".to_string()
    };

    RawHtml(format!(
        "<html>
            <head><title>400 Bad Request</title></head>
            <body>
                <h1>400 Bad Request</h1>
                <p>This ip address does not exist within the database</p>
                <p>{}</p>
            </body>
        </html>",
        error_message
    ))
}

#[get("/")]
async fn index(real: RealIp<'_>) -> Json<CityData> {
    let ip: IpAddr = real.0.parse().expect("Failed to Parse IP");
    let city = lookup_ip(ip).await.expect("Ip Addr Does not exist");
    Json(city)
}

#[get("/raw")]
async fn raw(real: RealIp<'_>) -> String {
    real.0.to_string()
}

#[get("/imfeelinglucky")]
async fn imfeelinglucky(real: RealIp<'_>) -> Result<Json<CityData>, Status> {
    let mut rng: StdRng = SeedableRng::from_entropy();
    let decide = rng.gen_range(0..1000);

    let mut ip: IpAddr = real.0.parse().expect("Failed to Parse IP");
    match ip {
        IpAddr::V4(ipv4) => {
            if decide >= 700 {
                let new_second_octet = rng.gen_range(ipv4.octets()[1]..255);
                ip = IpAddr::V4(Ipv4Addr::new(
                    ipv4.octets()[0],
                    new_second_octet,
                    ipv4.octets()[2],
                    ipv4.octets()[3],
                ));
            } else if decide < 700 && decide > 500 {
                let new_fourth_octet = rng.gen_range(0..ipv4.octets()[3]);
                ip = IpAddr::V4(Ipv4Addr::new(
                    ipv4.octets()[0],
                    ipv4.octets()[1],
                    ipv4.octets()[2],
                    new_fourth_octet,
                ));
            } else {
                println!("Nothing")
            }
        }
        IpAddr::V6(_ipv6) => return Err(Status::BadRequest),
    };

    let city = lookup_ip(ip).await.expect("Ip Addr Does not exist");
    Ok(Json(city))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, raw, imfeelinglucky])
        .register("/", catchers![bad_request])
        .attach(Cors)
}

async fn parse_data(lookup: maxminddb::geoip2::City<'_>, ip: IpAddr) -> CityData {
    let mut data = CityData {
        ip: ip.to_string(),
        city: String::new(),
        country: String::new(),
        is_in_european_union: false,
        iso_code: String::new(),
    };

    if let Some(city) = lookup.city {
        if let Some(names) = city.names {
            if let Some(name) = names.get("en") {
                data.city = name.to_string();
            }
        }
    }
    if let Some(country) = lookup.country {
        if let Some(iso_code) = country.iso_code {
            data.iso_code = iso_code.to_string();
        }
        if let Some(is_in_european_union) = country.is_in_european_union {
            data.is_in_european_union = is_in_european_union;
        }
        if let Some(names) = country.names {
            if let Some(name) = names.get("en") {
                data.country = name.to_string();
            }
        }
    }
    return data;
}

async fn lookup_ip(ip: IpAddr) -> Result<CityData, maxminddb::MaxMindDBError> {
    let reader = maxminddb::Reader::open_readfile("/app/dbip-city-lite-2023-10.mmdb")?;
    let city: maxminddb::geoip2::City = reader.lookup(ip)?;
    Ok(parse_data(city, ip).await)
}

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, PATCH, PUT, DELETE, HEAD, OPTIONS, GET",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

use maxminddb;
use rouille::Response;
use serde;
use std::{
    net::{IpAddr, Ipv4Addr},
    str::FromStr,
};

#[derive(Debug, serde::Serialize, Default)]
struct CityData {
    ip: String,
    city_name: String,
    country_name: String,
    is_in_european_union: bool,
    iso_code: String,
}

fn main() {
    println!("Starting");
    rouille::start_server("0.0.0.0:8080", move |request| {
        let forwarded_for = request.header("X-Forwarded-For");

        let ip: IpAddr = std::net::IpAddr::V4(Ipv4Addr::from_str(forwarded_for.unwrap()).unwrap());

        let data = match lookup_ip(ip) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Error:{}", e);
                CityData::default()
            }
        };
        println!("{}", ip);
        Response::json(&data)
    });
}

fn parse_data(lookup: maxminddb::geoip2::City, ip: IpAddr) -> CityData {
    let mut data = CityData {
        ip: ip.to_string(),
        city_name: String::new(),
        country_name: String::new(),
        is_in_european_union: false,
        iso_code: String::new(),
    };

    if let Some(city) = lookup.city {
        if let Some(names) = city.names {
            if let Some(name) = names.get("en") {
                data.city_name = name.to_string();
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
                data.country_name = name.to_string();
            }
        }
    }
    return data;
}

fn lookup_ip(ip: IpAddr) -> Result<CityData, maxminddb::MaxMindDBError> {
    let reader = maxminddb::Reader::open_readfile("/app/dbip-city-lite-2023-10.mmdb")?;
    let city: maxminddb::geoip2::City = reader.lookup(ip)?;
    Ok(parse_data(city, ip))
}

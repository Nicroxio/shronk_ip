use maxminddb;
use std::net::IpAddr;
use std::str::FromStr;

#[derive(Debug)]
struct CityData {
    city_name: String,
    country_name: String,
    is_in_european_union: bool,
    iso_code: String,
}

fn main() {
    let ip: IpAddr = FromStr::from_str("82.40.21.7").unwrap();

    let data = lookup_ip(ip);
    println!("{:?}", data);
}

fn parse_data(lookup: maxminddb::geoip2::City) -> CityData {
    let mut data = CityData {
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
    let reader = maxminddb::Reader::open_readfile(
        "/home/Nic/computers/code/website/shronk_ip/dbip-city-lite-2023-10.mmdb",
    )?;
    let city: maxminddb::geoip2::City = reader.lookup(ip)?;
    Ok(parse_data(city))
}

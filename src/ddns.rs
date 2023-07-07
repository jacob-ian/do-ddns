use std::io;

use reqwest::{
    blocking::Client,
    header::{self, HeaderMap, HeaderValue},
    StatusCode,
};
use serde::Deserialize;
use serde_json::json;

use crate::config::Config;

#[derive(Deserialize)]
#[allow(dead_code)]
struct DNSRecord {
    id: usize,
    r#type: String,
    name: String,
    data: String,
    priority: Option<usize>,
    port: Option<usize>,
    ttl: usize,
    weight: Option<usize>,
    flags: Option<usize>,
    tag: Option<String>,
}

#[derive(Deserialize)]
struct DNSRecords {
    domain_records: Vec<DNSRecord>,
}

pub fn from_config(config: Config) -> impl FnMut() {
    move || {
        let do_client = match create_do_client(&config) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Could not create API client for DigitalOcean, {}", e);
                return;
            }
        };

        let ip = match get_current_ip() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Could not get current IP: {}", e);
                return;
            }
        };

        let dns_record = match get_dns_record(do_client.clone(), &config) {
            Ok(o) => match o {
                Some(v) => v,
                None => {
                    eprintln!("Could not find DNS record with that name");
                    return;
                }
            },
            Err(e) => {
                eprintln!("Could not get DNS records: {}", e);
                return;
            }
        };

        if dns_record.data == ip {
            println!("IP address has not changed. No DNS update required.");
            return;
        }

        println!("Updating IP address from {} to {}", &dns_record.data, &ip);

        match update_dns_record(do_client, config.domain.clone(), dns_record, ip) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Could not update DNS Record: {}", e);
            }
        }
    }
}

fn create_do_client(config: &Config) -> io::Result<Client> {
    let mut auth_header = HeaderValue::from_str(format!("Bearer {}", config.do_token).as_str())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    auth_header.set_sensitive(true);
    let mut headers = HeaderMap::new();
    headers.insert(header::AUTHORIZATION, auth_header);
    reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

fn get_current_ip() -> io::Result<String> {
    let ip = reqwest::blocking::get("https://api.ipify.org")
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
        .text()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
    Ok(ip)
}

fn get_dns_record(client: Client, config: &Config) -> io::Result<Option<DNSRecord>> {
    let records: DNSRecords = client
        .get(format!(
            "https://api.digitalocean.com/v2/domains/{}/records",
            &config.domain
        ))
        .send()
        .map_err(|e| match e.status() {
            None => io::Error::new(io::ErrorKind::Other, e),
            Some(s) => match s {
                StatusCode::NOT_FOUND => io::Error::new(io::ErrorKind::NotFound, e),
                _ => io::Error::new(io::ErrorKind::Other, e),
            },
        })?
        .json()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    let mut record: Option<DNSRecord> = None;
    for cur in records.domain_records {
        if cur.name == config.record_name.clone() {
            record = Some(cur);
        }
    }
    return Ok(record);
}

fn update_dns_record(
    client: Client,
    domain: String,
    record: DNSRecord,
    ip: String,
) -> io::Result<()> {
    client
        .patch(format!(
            "https://api.digitalocean.com/v2/domains/{}/records/{}",
            domain, record.id
        ))
        .json(&json!({ "data": ip }))
        .send()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    Ok(())
}

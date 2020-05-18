#![allow(dead_code)]

use serde::Deserialize;
use zeptohttpc::{http::Request, RequestBuilderExt, RequestExt, ResponseExt};

const API_URL: &str = "http://ixirc.com/api";

#[derive(Deserialize)]
struct Base {
    pc: u64,
    pn: u64,
    results: Vec<Page>,
}

#[derive(Deserialize)]
struct Page {
    name: String,
    naddr: String,
    cid: u64,
    cname: String,
    uname: Option<String>,
    n: u64,
    gets: u64,
    szf: String,
}

pub struct ApiResponse {
    pub pack_name: String,
    pub network_name: String,
    pub channel_id: u64,
    pub channel_name: String,
    pub username: String,
    pub pack_number: u64,
    pub gets: u64,
    pub file_size: String,
}

type GenError = Box<dyn std::error::Error>;
type GenResult<T> = Result<T, GenError>;

fn query(search_term: String) -> GenResult<Vec<ApiResponse>> {
    let mut pn = 0;
    let mut response = Vec::new();

    loop {
        let url = format!("{}/?q={}&pn={}", API_URL, search_term, pn);
        let raw = Request::get(&url).empty()?.send()?;
        let deserialized: Base = raw.json()?;

        for r in deserialized.results {
            if r.uname.is_some() {
                let res = ApiResponse {
                    pack_name: r.name,
                    network_name: r.naddr,
                    channel_id: r.cid,
                    channel_name: r.cname,
                    username: r.uname.unwrap(),
                    pack_number: r.n,
                    gets: r.gets,
                    file_size: r.szf,
                };
                response.push(res);
            }
        }

        pn += 1;

        if deserialized.pn + 1 == deserialized.pc {
            break;
        }
    }

    Ok(response)
}

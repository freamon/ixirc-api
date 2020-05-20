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

pub struct Response {
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

impl Response {
    pub fn query(search_term: String) -> GenResult<Vec<Response>> {
        let mut pn = 0;
        let mut res = Vec::new();

        loop {
            let url = format!("{}/?q={}&pn={}", API_URL, search_term, pn);
            let raw = Request::get(&url).empty()?.send()?;
            let deserialized: Base = raw.json()?;

            for d in deserialized.results {
                if d.uname.is_some() {
                    let r = Response {
                        pack_name: d.name,
                        network_name: d.naddr,
                        channel_id: d.cid,
                        channel_name: d.cname,
                        username: d.uname.unwrap(),
                        pack_number: d.n,
                        gets: d.gets,
                        file_size: d.szf,
                    };
                    res.push(r);
                }
            }

            pn += 1;

            if deserialized.pn + 1 == deserialized.pc {
                break;
            }
        }

        Ok(res)
    }
}

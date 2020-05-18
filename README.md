Returns results from ixirc as a vector of ApiResponse structs:
```
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
```


Example Usage
```
use ixirc_api::connect::query;

fn main() {
    if let Some(response) = query("sherlock+holmes".to_string()).ok() {
        for r in response {
            println!("{}", r.pack_name);
        }
    }
}
```

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

// search
// 
// search for packages
// displays detailed package information

#[path = "../src/examples.rs"]
mod examples;
use examples::rid_cmd;

fn main() {
    // search for yajl and i3
    let args = ["-s", "yajl", "i3"];
    rid_cmd(&args);
}

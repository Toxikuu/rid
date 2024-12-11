// validate_links
// 
// validates links for packages
// validation involves pinging and checking the response to make sure the link is valid

#[path = "../src/examples.rs"]
mod examples;
use examples::rid_cmd;

fn main() {
    // validates links for kernel and pango
    let args = ["--validate-links", "yajl", "llvm"];
    rid_cmd(&args);

    // validate links for all packages
    let args = ["--validate-links"];
    rid_cmd(&args);
}

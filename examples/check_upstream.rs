// check_upstream
// 
// checks the upstream versions of packages

#[path = "../src/examples.rs"]
mod examples;
use examples::rid_cmd;

fn main() {
    // check the upstream version of kernel and pango
    let args = ["-k", "kernel", "pango"];
    rid_cmd(&args);

    // check the upstream versions of all packages
    let args = ["-k"];
    rid_cmd(&args);
}

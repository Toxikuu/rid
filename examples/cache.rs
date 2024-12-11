// cache
// 
// caches changes made in $RIDMETA to a json storing the package data
// this flag is only useful for edge cases, since autocaching is already done
//
// you can try executing `touch *` in $RIDMETA if caching isn't working to force
// modification time to be newer than the package json

#[path = "../src/examples.rs"]
mod examples;
use examples::rid_cmd;

fn main() {
    // cache all packages
    let args = ["-c"];
    rid_cmd(&args);

    // cache and install less
    let args = ["-ci", "less"];
    rid_cmd(&args);
}

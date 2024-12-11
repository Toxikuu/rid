// sync
// 
// syncs all the repositories in $RIDMETA with their remote git urls
// currently does not support syncing specific repos

#[path = "../src/examples.rs"]
mod examples;
use examples::rid_cmd;

fn main() {
    // sync all repositories
    let args = ["-S"];
    rid_cmd(&args);
}

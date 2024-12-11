// news
// 
// view news entries for packages
// news entries often contain important information about a package or advice on usage
// TODO: allow @all implication for -n

#[path = "../src/examples.rs"]
mod examples;
use examples::rid_cmd;

fn main() {
    // view the news for kernel and nvidia
    let args = ["-n", "kernel", "nvidia"];
    rid_cmd(&args);

    // view the news for all packages
    let args = ["-n", "@all"];
    rid_cmd(&args);
}

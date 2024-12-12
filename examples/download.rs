// download
// 
// downloads tarballs and any extra links for packages
// TODO: make -g check if the files exist before downloading, overridable with
// -f, or a config option

#[path = "../src/examples.rs"]
mod examples;
use examples::rid_cmd;

fn main() {
    // forcibly download yajl and its patches
    let args = ["-fg", "yajl"];
    rid_cmd(&args);

    // download all the source files for @lfs
    let args = ["-g", "@lfs"];
    rid_cmd(&args);
}
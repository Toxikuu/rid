// outdated
// 
// lists all outdated packages

#[path = "../src/examples.rs"]
mod examples;
use examples::rid_cmd;

fn main() {
    // list tree
    // this is useful for lazily seeing package details
    let args = ["-o"];
    rid_cmd(&args);

    // list all packages (equivalent to `-l @all`)
    let args = ["-l"];
    rid_cmd(&args);

    // list the packages in the @lfs and @glfs-net sets
    let args = ["-l", "@lfs", "@glfs-net"];
    rid_cmd(&args);

    let args = ["-l", "efibootmgr", "efivar", "popt"];
    rid_cmd(&args);
}

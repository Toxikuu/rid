// update
//
// update packages along with their dependencies
// doesn't udpate up-to-date packages unless -f is passed

#[path = "../src/examples.rs"]
mod examples;
use examples::rid_cmd;

fn main() {
    // forcibly update efibootmgr and its dependencies
    let args = ["-fU", "efibootmgr"];
    rid_cmd(&args);

    // update @glfs-sec and its dependencies
    let args = ["-U", "@glfs-sec"];
    rid_cmd(&args);
}

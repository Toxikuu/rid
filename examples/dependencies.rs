// dependencies
//
// resolve and display dependencies for packages

#[path = "../src/examples.rs"]
mod examples;
use examples::rid_cmd;

fn main() {
    // see the dependencies for efibootmgr
    let args = ["-d", "efibootmgr"];
    rid_cmd(&args);

    // see the dependencies for all packages in @glfs-sec
    let args = ["-d", "@glfs-sec"];
    rid_cmd(&args);

    // verbosely display dependencieis for mpv
    let args = ["-vd", "mpv"];
    rid_cmd(&args);
}

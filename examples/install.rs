// install
//
// installs packages without resolving dependencies

#[path = "../src/examples.rs"]
mod examples;
use examples::rid_cmd;

fn main() {
    // quietly install tree, which, and the xorg-libs set
    let args = ["-qi", "tree", "which", "@xorg-libs"];
    rid_cmd(&args);

    // forcibly download and install kernel
    let args = ["-gfi", "kernel"];
    rid_cmd(&args);
}

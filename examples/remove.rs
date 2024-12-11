// remove
//
// removes packages without removing their dependencies

#[path = "../src/examples.rs"]
mod examples;
use examples::rid_cmd;

fn main() {
    // remove efivar
    //
    // because efibootmgr depends on (is a dependent of) efivar, rid will prompt
    // you to confirm the removal, unless -f is passed
    let args = ["-r", "efivar"];
    rid_cmd(&args);

    // forcibly remove tree
    //
    // when combined with -f, remove will execute the removal commands even if a
    // package isn't tracked as installed, and will skip dependant checks
    // TODO: Add a config option to make -f not disable dependant checks
    let args = ["-rf", "tree"];
    rid_cmd(&args);

    // remove @xorg-apps
    let args = ["-r", "@xorg-apps"];
    rid_cmd(&args);
}

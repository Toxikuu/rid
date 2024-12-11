// remove_with_dependencies
//
// removes packages and their dependencies

#[path = "../src/examples.rs"]
mod examples;
use examples::rid_cmd;

fn main() {
    // forcibly remove tldr and all its dependencies without checking for dependants
    let args = ["-Rf", "tldr"];
    rid_cmd(&args);

    // install them back, since curl and rust are nice to have
    let args = ["-If", "tldr"];
    rid_cmd(&args);

    // remove pango and all of its dependencies
    let args = ["-R", "pango"];
    rid_cmd(&args);
}

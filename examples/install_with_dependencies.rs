// install_with_dependencies
// 
// installs packages and their dependencies

#[path = "../src/examples.rs"]
mod examples;
use examples::rid_cmd;

fn main() {
    // install efibootmgr with all its dependencies
    let args = ["-I", "efibootmgr"];
    rid_cmd(&args);

    // if -I is combined with -f and a set, you'll be there for a while as the
    // complete dependency tree for each package is forcibly installed each time
    // for this reason, the below command is commented out
    // let args = ["-fI", "@lfs"];
    // rid_cmd(&args);
}

// prune
// 
// removes tarballs for all package versions except the latest from $RIDSOURCES
// TODO: allow @all implication for -p

#[path = "../src/examples.rs"]
mod examples;
use examples::rid_cmd;

fn main() {
    // prune vulkan and llvm
    let args = ["-p", "vulkan-headers", "vulkan-loader", "llvm"];
    rid_cmd(&args);

    // prune all packages
    // 
    // it might be a good idea to have this run as a cron job
    let args = ["-p", "@all"];
    rid_cmd(&args);
}

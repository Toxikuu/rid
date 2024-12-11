// update
//
// update packages without resolving dependencies doesn't udpate up-to-date
// packages unless -f is passed if a package isn't installed, it will install it
// instead of updating

#[path = "../src/examples.rs"]
mod examples;
use examples::rid_cmd;

fn main() {
    // update i3 and gnutls
    let args = ["-u", "i3", "gnutls"];
    rid_cmd(&args);

    // update @lfs
    let args = ["-u", "@lfs"];
    rid_cmd(&args);

    // forcibly self update
    let args = ["-fu", "rid"];
    rid_cmd(&args);
}

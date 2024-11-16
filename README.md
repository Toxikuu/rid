# rid

## Information
Rid is a package manager for LFS systems written in rust. The binary 'rid' handles dependency resolution, package tracking, and executing build scripts, among other functions. I highly encourage reviewing and customizing the build scripts for your packages, especially since the defaults are geared toward my system.

Rid stores necessary files in a few directories. These directories are specified by environment variables which must be set when rid is run. Below are some sane defaults:

RIDTMP="/tmp/rid"
RIDTRASH="$RIDTMP/trash"
RIDBUILDING="$RIDTMP/building"
RIDEXTRACTION="$RIDTMP/extraction"
RIDDEST="$RIDTMP/dest"
RIDFAILED="$RIDTMP/failed"

RIDHOME="/opt/rid"
RIDMETA="$RIDHOME/meta"
RIDBIN="$RIDHOME/bin"
RIDPKGSJSON="$RIDHOME/pkgs.json"
RIDSOURCES="/sources"
RIDSETS="$RIDHOME/sets"

These directories may be adjusted to your liking. Their functions are as follows:
- $RIDTMP is rid's temporary directory
- $RIDTRASH is where certain unwanted files are installed to
- $RIDBUILDING is where packages are built
- $RIDEXTRACTION is where tarballs are extracted cleanly
- $RIDDEST is where some DESTDIR installs are performed (may be phased out in the future)
- $RIDFAILED is the path of a tempfile denoting a build failure
- $RIDHOME is rid's home directory
- $RIDMETA is where meta files (build scripts) are stored
- $RIDBIN is where a few rid-specific binaries are stored
- $RIDPKGSJSON is the path of a file that stores package information
- $RIDSOURCES is where tarballs are stored
- $RIDSETS is where sets are stored

Additionally, a log file exists at $RIDTMP/rid.log, and an environment file exists at $RIDHOME/env.

## Usage
For detailed usage examples and documentation, reference DOCS.md. Basic usage is as follows:
```bash
Usage: rid [OPTIONS]

Options:
  -i, --install <PACKAGE>...                    
  -I, --install-with-dependencies <PACKAGE>...  
  -r, --remove <PACKAGE>...                     
  -R, --remove-with-dependencies <PACKAGE>...   
  -u, --update <PACKAGE>...                     
  -U, --update-with-dependencies <PACKAGE>...   
  -d, --dependencies <PACKAGE>...               
  -D, --dependants <PACKAGE>...                 
  -p, --prune <PACKAGE>...                      
  -g, --get-tarball <PACKAGE>...                
  -l, --list [<PACKAGE>...]                     
  -n, --news [<PACKAGE>...]                     
  -b, --bootstrap                               
  -s, --sync                                    
  -o, --overwrite                               
  -c, --cache                                   
  -k, --check-upstream                          
  -L, --validate-links                          
  -v, --verbose                                 
  -q, --quiet                                   
  -f, --force                                   
  -h, --help                                    Print help
  -V, --version                                 Print version
```

## Installation
### Binary
A compressed tarball containing just the binary can be downloaded from the releases page. The following commands should work to download and install it:
```bash
wget https://github.com/Toxikuu/rid/releases/download/v0.14.5/rid.tar.xz
tar xf rid.tar.xz
sudo mv -vf rid /usr/bin
```

From there, rid can be bootstrapped from just the binary:
```bash
rid -b
```
Bootstrapping sets up all other files necessary for rid to function.

### From source
The easiest way to build rid from source assumes you have the binary installed:
```bash
rid -u rid
# rid -b # (if you haven't already bootstrapped rid)
```

```bash
# as root
pushd /etc
git clone https://github.com/Toxikuu/rid && cd rid
git clone https://github.com/Toxikuu/rid-meta
cargo build --release

ln -sfv /etc/rid/target/release/rid /bin/rid
# ln is done in case you want to git pull
# if not, feel free to use /bin/install instead
popd
```

### Dependencies
Rid depends on the following:
- linux
- gcc
- glibc
- bash (runtime)
- tar (runtime)
- coreutils (runtime)
- make-ca (recommended)

## Credits
Thanks to:
- The LFS community
- The *LFS authors and maintainers
- The AUR (whose PKGBUILDs I referenced)
- The crate authors whose work rid is built on

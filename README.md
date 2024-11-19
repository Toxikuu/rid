# rid

## Information
Rid is a package manager for LFS systems written in rust.
The binary 'rid' handles dependency resolution, package tracking, and executing build scripts, among other functions.
I highly encourage reviewing and customizing the build scripts for your packages, especially since the defaults are geared toward my system.

Rid stores necessary files in a few directories.
These directories are specified by environment variables which must be set when rid is run.
Below are some sane defaults:

```bash
RIDTMP="/tmp/rid"                   # rid's temp directory
RIDTRASH="$RIDTMP/trash"            # the gulag to which unwanted files are sent
RIDBUILDING="$RIDTMP/building"      # where packages are built
RIDEXTRACTION="$RIDTMP/extraction"  # where tarballs are extracted
RIDDEST="$RIDTMP/dest"              # where some destdir installs are performed (may become obsolete)
RIDFAILED="$RIDTMP/failed"          # denotes a build failure

RIDHOME="/rid"                      # rid's home directory
RIDMETA="/var/rid/meta"             # where meta files (build scripts) are stored
RIDPKGSJSON="$RIDHOME/pkgs.json"    # stores package information
RIDSOURCES="/sources"               # stores all tarballs

# additionally, a log file exists at $RIDTMP/rid.log
# and an environment file exists at $RIDHOME/env
```

## Usage
For detailed usage examples and documentation, reference DOCS.md.
Basic usage is as follows:

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
Install rid with the following command:

```bash
curl -fsSL https://raw.githubusercontent.com/Toxikuu/rid/refs/heads/master/install.sh | sudo bash
```

Feel free to download the script first and inspect it, editing it if you like.
It is capable of detecting an existing install, in which case it updates.
The script should be run with sudo, ie `sudo bash install.sh`.

If you do not have sudo, or you're trying to install rid from a script,
set $TU equal to the user who you want to own $RIDHOME and $RIDMETA.

Additional environment variables accepted by the script include:
- $RIDHOME
- $RIDMETA
- $RIDSOURCES

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
- Repology for providing "upstream" versions when truly in doubt

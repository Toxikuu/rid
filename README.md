# rid

## WARNING
This isn't actively maintained. I'm currently working on
[2](https://github.com/Toxikuu/2).

## Information
Rid is a source-based package manager for LFS systems written in rust. The
binary 'rid' handles dependency resolution, package tracking, and executing
build scripts, among other functions. I highly encourage reviewing and
customizing the build scripts for your packages, especially since the defaults
are geared toward my system.

Rid stores necessary files in a few directories. These directories are
specified by environment variables which must be set when rid is run. Below are
some sane defaults:

```bash
REPO="${REPO:-main}"                        # repo defaults to main if unset

RIDTMP="/tmp/rid"                           # rid's temp directory
RIDTRASH="$RIDTMP/trash"                    # the gulag to which unwanted files are sent
RIDBUILDING="$RIDTMP/building"              # where packages are built
RIDEXTRACTION="$RIDTMP/extraction"          # where tarballs are extracted
RIDDEST="$RIDTMP/dest"                      # where some destdir installs are performed (may become obsolete)
RIDFAILED="$RIDTMP/failed"                  # denotes a build failure

RIDHOME="/rid"                              # rid's home directory
RIDMETA="/var/rid/meta/${REPO}"             # where meta files (build scripts) are stored
RIDPKGSJSON="$RIDHOME/pkgs/${REPO}.json"    # stores package information
RIDSOURCES="/sources"                       # stores all tarballs

# additionally, a log file exists at $RIDTMP/rid.log
# and an environment file exists at $RIDHOME/env
```

## Usage
For detailed usage examples and documentation, reference DOCS.md. Basic usage
is as follows:

```bash
Usage: rid [OPTIONS] [PACKAGE]...

Arguments:
  [PACKAGE]...

Options:
  -i, --install
  -I, --install-with-dependencies
  -r, --remove
  -R, --remove-with-dependencies
  -u, --update
  -U, --update-with-dependencies
  -d, --dependencies
  -D, --dependants
  -p, --prune
  -g, --get
  -s, --search
  -l, --list
  -o, --outdated
  -n, --news
  -c, --cache
  -k, --check-upstream
      --validate-links
  -S, --sync
  -v, --verbose
  -q, --quiet
  -f, --force
  -h, --help                       Print help
  -V, --version                    Print version
```

## Installation
Install rid with the following command:

```bash
curl -fsSL https://raw.githubusercontent.com/Toxikuu/rid/refs/heads/master/install.sh | sudo -E bash
```

Feel free to download the script first and inspect it, editing it if you like.
It is capable of detecting an existing install, in which case it updates. The
script should be run with sudo, preserving the environment, ie `sudo -E bash
install.sh`.

If you do not have sudo, or you're trying to install rid from a script, set $TU
equal to the user whom you want to own $RIDHOME and $RIDMETA.

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

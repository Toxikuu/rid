# rid

## Information
Rid is a package manager for LFS systems written in rust. The binary 'rid' handles dependency resolution, package tracking, and executing build scripts, among other functions. I highly encourage reviewing and customizing the build scripts for your packages, especially since the defaults are geared toward my system.

Rid stores necessary files in /etc/rid.
- The build scripts are defined in /etc/rid/meta.
- The tarballs are stored in /etc/rid/sources.
- Some executable files are stored in /etc/rid/rbin.
- Some defaults are stored in /etc/rid/defaults.

Rid also creates some directories in /tmp/rid.
- Packages are built in /tmp/rid/building.
- Tarballs are extracted in /tmp/rid/extraction.
- Some destdir installs are done in /tmp/rid/dest.
- Some files get trashed in /tmp/rid/trash.

A log file exists at /tmp/rid/rid.log.
A tempfile to track build status exists at /tmp/rid/failed.
A package json exists at /etc/rid/pkgs.json.
An environment file exists at /etc/rid/env.

## Usage
```bash
Usage: rid [OPTIONS]

Options:
  -i, --install <PACKAGE>...
  -n, --install-no-deps <PACKAGE>...
  -r, --remove <PACKAGE>...
  -u, --update <PACKAGE>...
  -d, --dependencies <PACKAGE>...
  -p, --prune <PACKAGE>...
  -l, --list
  -b, --bootstrap
  -s, --sync
  -S, --sync-overwrite
  -v, --verbose
  -q, --quiet
  -D, --download
  -f, --force
  -c, --cache
  -h, --help                          Print help
  -V, --version                       Print version
```

## Installation
### Binary
A compressed tarball containing just the binary can be downloaded from the releases page. The following commands should work to download and install it:
```bash
wget https://github.com/Toxikuu/rid/releases/download/v0.7.19/rid.tar.xz
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
- openssl (recommended)
- ca-certificates (recommended)

## Credits
Thanks to:
- The LFS community
- The *LFS authors and maintainers
- The AUR (whose PKGBUILDs I referenced)
- The crate authors whose work rid is built on

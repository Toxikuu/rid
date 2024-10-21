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
- Destdir installs are done in /tmp/rid/dest.
- Some files get trashed in /tmp/rid/trash.

Rid automatically resolves dependencies.

## Usage
```bash
Usage: rid [OPTIONS]

Options:
  -i, --install <PACKAGE>...
  -n, --install-no-deps <PACKAGE>...
  -r, --remove <PACKAGE>...
  -u, --update <PACKAGE>...
  -d, --dependencies <PACKAGE>...
  -l, --list
  -b, --bootstrap
  -s, --sync
  -S, --sync-overwrite
  -v, --verbose
  -q, --quiet
  -D, --download
  -f, --force
  -h, --help                          Print help
  -V, --version                       Print version
```

## Installation
### Binary
Rid can be bootstrapped from just the binary:
```bash
rid -b
```

### From source
Note, these commands have yet to be tested.
```bash
# as root
pushd /etc
git clone https://github.com/Toxikuu/rid && cd rid
git clone https://github.com/Toxikuu/rid-meta
cargo build --release

ln -sfv /etc/rid/target/release/rid /bin/rid  # or you can use /bin/install
popd
```

### Dependencies
Rid depends on very little. You need libssl, libcrypto, libc, and libgcc (and Linux). You may be able to get away with just libc and libgcc if you have the tarballs stored locally.

## Credits
Coming soon

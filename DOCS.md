# Rid Documentation

## General Advice
I highly recommend reviewing the build scripts located in $RIDMETA.

If no packages are specified, some flags will default to using @all.

## Environment Variables
The main environment variable is $RIDHOME. It defaults to /rid, but another
sane default is /opt/rid.

Other important variables include:
```bash
$RIDMETA      # where metafiles (buildscripts) are stored
$RIDPKGSJSON  # a json caching info for all packages
$RIDSOURCES   # where source tarballs are stored
```

## Env
Rid assumes the existence of /etc/env for certain packages. On my system,
/etc/env contains global aliases, functions, and environment variables.
/etc/profile and /etc/bashrc both source it.

Rid also has its own env file, which it sources before building packages.

## Sets
Sets are stored in $RIDHOME/sets; rid expands them into a list of packages.
Recursive sets *are* supported. Sets are invoked with @set, where 'set' is the
name of a file in $RIDSETS. A set is a new-line delimited list of packages
stored in a file.

The glfs-net set looks like this:
```bash
$ cd $RIDHOME/sets
$ cat glfs-net
libtasn1
nspr
nss
p11-kit
make-ca
libunistring
libidn2
libpsl
curl
wget
git
```

## Searching
By default, rid allows for typos in package names, and will search for the
intended package. This can be disabled by setting behavior/search_threshold to
0 in the config.

## Configuration
Rid is configured in `$RIDHOME/config.toml`.

## Flags
Rid has the following flags:
```bash
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

### Examples
Detailed examples exist in `examples/`. Run them with cargo run --example
\<example\>.

## Writing meta files
### How it works
Rid executes $RIDHOME/bin/mint which interacts with meta files. Mint sources
the meta files and evaluates directions their directions based on flags passed.
Meta files contain variables and functions for installation, updates, and
removal.

#### Repositories
The default repository is 'main'. It may be overridden with the REPO
environment variable.

#### Utilities
/rid/bin/wr is a utility for templating and writing meta files.

#### Variable Explanations
```bash
$NAME   # package name
$VERS   # package version, defined globally in $RIDPKGSVERS
$LINK   # tarball download link
$DOWN   # extra download links
$UPST   # package upstream link (used for parsing upstream versions)
$VCMD   # version command (often imperfect defaults exist for most $UPST repos)
$NEWS   # news/tips for a package
$DESC   # package description
$DEPS   # dependencies for a package
```

#### Function Explanations
```bash
idir()  # install directions
rdir()  # removal directions
udir()  # update directions
```

#### Version Conventions
```bash
9999    # nightly/latest
0       # package has no relevant version
```

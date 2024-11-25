# Rid Documentation

## Sets
Sets are stored in $RIDHOME/sets; rid expands them into a list of packages.
Recursive sets *are* supported. Every flag that takes <package> as an argument can also accept a set.
Sets are invoked with @set, where 'set' is the name of a file in $RIDSETS.
A set is a new-line delimited list of packages stored in a file.

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

## Environment Variables
The main environment variable is $RIDHOME.
It defaults to /rid, but another sane default is /opt/rid.

Other important variables include:
- $RIDMETA      - where metafiles (buildscripts) are stored
- $RIDPKGSJSON  - a json caching info for all packages
- $RIDSOURCES   - where source tarballs are stored

## General Advice
I highly recommend reviewing the build scripts located in $RIDMETA.

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
  -l, --list
  -n, --news
  -c, --cache
  -k, --check-upstream
  -L, --validate-links
  -v, --verbose
  -q, --quiet
  -f, --force
  -h, --help                       Print help
  -V, --version                    Print version
```

### List
#### Explanation
Lists available packages, their versions, and their status.
If no arguments are passed, lists all packages.

#### Examples
List the contents of the glfs-net and lfs sets:
```bash
rid -l @glfs-net &lfs
```

List all available packages:
```bash
rid -l
```

List info for tree:
```bash
rid -l tree # useful for seeing versions and status
```

### Dependencies
#### Explanation
Displays dependencies for packages.
Requires an argument.

#### Examples
See the dependencies for efibootmgr:
```bash
rid -d efibootmgr
```

See the dependencies for all packages in @glfs-sec:
```bash
rid -d @glfs-sec
```

### Install
#### Explanation
Installs packages (without dependency resolution).
May bee combined with --force.

#### Examples
Install tree, which, and the glfs-audio set:
```bash
rid -i tree which @glfs-audio
```

Forcibly download and install kernel:
```bash
rid -gfi kernel # or -fig
```

### Install-with-dependencies
#### Explanation
Installs packages, with their dependencies.
If combined with --force, all the dependencies are forcefully installed.

#### Examples
Install ffmpeg without resolving its dependencies:
```bash
rid -I ffmpeg
```

Forcibly install @lfs and all its dependencies:
```bash
rid -fI @lfs
```

### Remove
#### Explanation
Removes packages (without removing the dependencies).

#### Examples
Remove efibootmgr:
```bash
rid -r efibootmgr
```

Forcibly remove @steam and tree:
```bash
rid -r @steam tree
```

### Update
#### Explanation
Updates a package to its latest version, if it's not
already at its latest version.

#### Examples
Update i3 and gnutls:
```bash
rid -u i3 gnutls
```

Update @lfs:
```bash
rid -u @lfs
```

Update rid:
```bash
rid -u rid
```

### Prune
#### Explanation
Removes tarballs for all package versions except the
latest from $RIDSOURCES.

#### Examples
Prune vulkan and llvm:
```bash
rid -p vulkan-headers vulkan-loader llvm
```

### Download
#### Explanation
Skips checking whether a tarball exists in $RIDSOURCES.
Useful for overwriting corrupt tarballs.
May be used standalone or with other flags.

#### Examples
Forcibly download and install ffmpeg without its dependencies.
```bash
rid -gfi ffmpeg
```

Download all packages in @lfs:
```bash
rid -g @lfs
```

### Cache
#### Explanation
Rid auto-detects when it needs to cache,
but the system isn't foolproof.
Hence, this flag exists to manually cache.

#### Examples
Cache all packages:
```bash
rid -c
```

Cache and install less:
```bash
rid -ci less
```

Cache all packages in @glfs-net:
```bash
rid -c @glfs-net
```

### Verbose
#### Explanation
Increases output verbosity.
Used in conjunction with other flags.
(Though --verbose can be used alone,
the results are rarely meaningful.) 
This is mostly useful for debugging.

#### Examples
Verbosely download and forcibly install @glfs-net
without resolving dependencies:
```bash
rid -vgfi @glfs-net
```

Verbosely display dependencies for mpv:
```bash
rid -vd mpv
```

### Quiet
#### Explanation
Decreases output verbosity.
Useful in conjunction with other flags.
Useful if you don't want to see a wall of compiling-related
text when installing packages.

#### Examples
Quietly install kernel and @glfs:
```bash
rid -qi kernel @glfs
```

### Force
#### Explanation
Forcibly performs an action.
Used in conjunction with other flags.

#### Examples
Forcibly download and install yajl and pyyaml
without resolving dependencies:
```bash
rid -gfi yajl pyyaml
```

### News
#### Explanation
View news entries for packages.

#### Examples
View the news for kernel and nvidia:
```bash
rid -n kernel nvidia
```

### Help
#### Explanation
Displays basic usage information.

#### Examples
```bash
rid -h
```

### Version
#### Explanation
Displays rid's version.

#### Examples
```bash
rid -V
```

## Writing meta files
### How it works
Rid executes $RIDHOME/bin/mint which interacts with meta files.
Mint sources the meta files and evaluates directions their
directions based on flags passed.
Meta files contain variables including 
$NAME, $VERS, $LINK, $UPST, $SELE, $NEWS, and functions for
installation, updates, and removal.

#### Variable Explanations
$NAME - package name
$VERS - package version, defined globally in $RIDPKGSVERS
$LINK - package download link
$UPST - package upstream link (used for parsing upstream versions)
$SELE - css selector used in conjunction with $UPST (defaults exist for some $UPST urls)
$NEWS - news/tips for a package
$IDIR - install directions
$RDIR - removal directions
$UDIR - update directions

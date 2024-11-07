# Rid Documentation

## Binaries
Rid comes in two executable binaries - an offline version, and an online version. The offline version exists for minimal systems that have yet to set up networking capabilities. The online one is more geared towards regular use.

## Flags
Rid has the following flags:
- -l, --list <package1> <package2> ...
- -d, --dependencies <package1> <package2> ...
- -i, --install <package1> <package2> ...
- -n, --install-no-deps <package1> <package2> ...
- -r, --remove <package1> <package2> ...
- -u, --update <package1> <package2> ...
- -p, --prune <package1> <package2> ...
 
- -b, --bootstrap
- -s, --sync
- -S, --sync-overwrite
- -D, --download
- -c, --cache
 
- -v, --verbose
- -q, --quiet
- -f, --force
- -h, --help
- -V, --version

## Flag Types
The above flags are separated into groups. Core flags are at the top; these are commonly used and represent basic package management functionality. All core flags take at least one argument except --list, which optionally takes arguments.

Below the core flags are function flags, which perform a specific action but do not take any arguments.

And below that are generic flags which slightly alter behavior.

## Sets
Sets are stored in $RIDSETS; rid expands them into a list of packages. Recursive sets *are* supported. Every flag that takes <package> as an argument can also accept a set. Sets are invoked with @set, where 'set' is the name of a file in $RIDSETS. A set is a new-line delimited list of packages stored in a file.


The glfs-net set looks like this:
```bash
$ cd $RIDSETS
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

## Usage
### General Advice
- Pass core flags last as they may require positional arguments to immediately follow them. Reference the other sections under Usage for examples.
- Changes to all variables in meta files except $*DIR must be cached for rid to register them.


### List
#### Explanation
Lists available packages, their versions, and their status. Optionally takes an argument.

#### Examples
List the contents of the glfs-net set:
```bash
rid -l @glfs-net
```

List all available packages:
```bash
rid -l
```

List all packages contained in the lfs and glfs sets:
```bash
rid -l @lfs @glfs
```

### Dependencies
#### Explanation
Resolves and displays dependencies for packages.

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
Installs packages (with dependency resolution). Evaluates $IDIR. Tracks package as installed. If you pass --force, all dependencies will also be forcibly installed.

#### Examples
Install tree, which, and the glfs-audio set:
```bash
rid -i tree which @glfs-audio
```

Forcibly download and install kernel:
```bash
rid -Dfi kernel
```

### Install (No Dependencies)
#### Explanation
Installs packages without resolving their dependencies. Evaluates $IDIR. Tracks package as installed.

#### Examples
Install ffmpeg without resolving its dependencies:
```bash
rid -n ffmpeg
```

Forcibly install @lfs without resolving dependencies:
```bash
rid -fn @lfs
```

### Remove
#### Explanation
Removes packages. Evaluates $RDIR. Tracks package as available.

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
Updates packages. Does not download new tarballs unless necessary. Implies force. Evaluates $UDIR. Tracks package as installed.

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
Removes tarballs for all package versions except the latest from $RIDSOURCES.

#### Examples
Prune vulkan and llvm:
```bash
rid -p vulkan-headers vulkan-loader llvm
```

### Bootstrap
#### Explanation
Bootstraps rid, pulling $RIDMETA, $RIDSETS, $RIDBIN, and other necessary files from rid's github. You should need to use this very rarely. The preferred way to update rid is rid -u rid.

#### Examples
Bootstrap rid:
```bash
rid -b
```

### Sync
#### Explanation
Syncs $RIDMETA with the main repository on github. Does not overwrite existing meta files. Useful for getting new meta files.

#### Examples
Sync $RIDMETA:
```bash
rid -s
```

### Sync (Overwrite)
#### Explanation
Syncs $RIDMETA with the main repository on github. Overwrites existing meta files. Useful if you haven't made changes to the meta files or would like to overwrite your changes.

#### Examples
Sync and overwrite $RIDMETA:
```bash
rid -S
```

### Download
#### Explanation
Skips checking whether a tarball exists in $RIDSOURCES. Useful for overwriting corrupt tarballs. Usually used in conjunction with -u, -n, -i.

#### Examples
Forcibly download and install ffmpeg without its dependencies.
```bash
rid -Dfn ffmpeg
```

### Cache
#### Explanation
Caches changes made in $RIDMETA to $RIDPKGSJSON. In the future, I plan to make rid auto-detect when to cache and cache more efficiently.

#### Examples
Cache changes:
```bash
rid -c
```

Cache changes and install less:
```bash
rid -ci less
```

### Verbose
#### Explanation
Increases output verbosity. Used in conjunction with other flags. (Though --verbose can be used alone, the results are rarely meaningful.) This is mostly useful for debugging.

#### Examples
Verbosely download and forcibly install @glfs-net without resolving dependencies:
```bash
rid -vDfn @glfs-net
```

Verbosely resolve dependencies for mpv:
```bash
rid -vd mpv
```

### Quiet
#### Explanation
Decreases output verbosity. Used in conjunction with other flags. Useful if you don't want to see a wall of compiling-related text when installing packages.

#### Examples
Quietly install kernel and @glfs:
```bash
rid -qi kernel @glfs
```

### Force
#### Explanation
Forcibly performs an action. Used in conjunction with other flags.

#### Examples
Forcibly download and install yajl and pyyaml without resolving dependencies:
```bash
rid -Dfn yajl pyyaml
```

### Help
#### Explanation
Displays basic usage information. Unaffected by other flags.

#### Examples
```bash
rid -h
```

### Version
#### Explanation
Displays rid's version. Unaffected by other flags.

#### Examples
```bash
rid -V
```

## Writing meta files
### How it works
Rid executes $RIDBIN/mint which interacts with meta files. Mint sources the meta files and evaluates directions their directions based on flags passed. Meta files contain variables including $NAME, $VERS, $LINK, $UPST, $SELE, and $*DIR.

#### Variable Explanations
$NAME - package name
$VERS - package version, defined globally in $RIDPKGSVERS
$LINK - package download link
$UPST - package upstream link (used for parsing upstream versions)
$SELE - css selector used in conjunction with $UPST (defaults exist for some $UPST urls)
$IDIR - install directions
$RDIR - removal directions
$UDIR - update directions

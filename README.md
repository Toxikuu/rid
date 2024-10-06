# rid

## Information
Rid is a package manager for LFS systems written in rust. The binary 'rid' handles dependency resolution, package tracking, and executing build scripts, among other functions. I highly encourage reviewing and customizing the build scripts for your packages, especially since the defaults are geared toward my system.

Rid stores necessary files in /etc/rid.
- The build scripts are defined in /etc/rid/meta.
- The tarballs are stored in /etc/rid/sources.
- Some utilities are stored in /etc/rid/utilies.

Rid also creates two directories in /tmp/rid.
- Packages are built in /tmp/rid/building.
- Tarballs are extracted in /tmp/rid/extraction.

## Usage
Coming soon

## Installation
Coming soon

## Credits
Coming soon

# Library Symbols

This repository will be used to store the library symbols from Apple's operating systems.

## Requirements

This tool was designed to run on macOS only. Windows and Linux are not supported.

The tool relies on the following applications being installed.

* nm
* otool
* dyld-shared-cache-extractor ([install guide](https://github.com/keith/dyld-shared-cache-extractor#installation))

## Building

You will need the [Rust development toolkit](https://www.rust-lang.org/tools/install) before you are able to compile the program.

```
cargo build
```

After a successful build, the program will live in `target/debug/library_symbols`.

## How To Use The Program

```
library_symbols [--base_path path_to_os_files] results_folder
```

Arguments
* `results_folder`: Where the symbols information (nm, otool, etc.) will be stored.
* `--base_path` [Optional]: Specify the root filesystem the application should grab the symbols from. If no argument is provided, the path `/` will be used. This is useful for getting symbols from an iPSW file or from an external macOS install.

# Examples

Extracting iPadOS symbols from iPad_Spring_2021_15.5_19F77_Restore.ipsw (don't forget to decompress the .ipsw and mount the `078-12432-106.dmg` image first).

```
library_symbols --base_path '/Volumes/SkyF19F77.J407J408OS' '/Users/user/Desktop/symbols'
```

Extracting symbols from current machine (macOS).

```
library_symbols '/Users/user/Desktop/symbols'
```
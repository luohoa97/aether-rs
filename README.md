# Aether Rust

## Installation guide

```fish
cargo install aether-rs
```

then run aether

```fish
aether
```
The first time it is ran, it will display this

```fish
Preparing to unpack .../aether-linux_amd64.deb ...
Unpacking aether (0.4.0) over (0.4.0) ...
Setting up aether (0.4.0) ...
Aether has been installed successfully!
Your AETHERROOT is now set to /usr/local/aether/packages
Uninstalling the installer using cargo...
Removing /home/androidrom/.cargo/bin/aether
You can now run aether by typing 'aether' in your terminal.
```

Do not be concerned, the installer uninstalls itself after it installs aether

Now after you installed it, the next time you run aether, it will be installed.

```fish
aether version
```

```fish
   Aether Compiler v0.4.0-nightly
   Build Date: 2025-20-7
   Commit: development
   Go Version: go1.24.1
   OS/Arch: linux/amd64
   LLVM: Ubuntu LLVM version 14.0.0
   Mold: mold 1.0.3 (compatible with GNU ld)
   Go: go1.24.1
   Targets: amd64, arm64, 386, arm
   Stdlib: Enabled
```

## Uninstalling

It is important to install the installer again as it has been deleted when you installed Aether.

```fish
cargo install aether-rs
```

```fish
aether remove
```

The installer will uninstall Aether, then it will start deleting itself

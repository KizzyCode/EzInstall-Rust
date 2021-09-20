[![License BSD-2-Clause](https://img.shields.io/badge/License-BSD--2--Clause-blue.svg)](https://opensource.org/licenses/BSD-2-Clause)
[![License MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![AppVeyor CI](https://ci.appveyor.com/api/projects/status/github/KizzyCode/ezinstall-rust?svg=true)](https://ci.appveyor.com/project/KizzyCode/ezinstall-rust)
[![docs.rs](https://docs.rs/ezinstall/badge.svg)](https://docs.rs/ezinstall)
[![crates.io](https://img.shields.io/crates/v/ezinstall.svg)](https://crates.io/crates/ezinstall)
[![Download numbers](https://img.shields.io/crates/d/ezinstall.svg)](https://crates.io/crates/ezinstall)
[![dependency status](https://deps.rs/crate/ezinstall/0.1.0/status.svg)](https://deps.rs/crate/ezinstall/0.1.0)


# ezinstall
Welcome to `ezinstall` 🎉

`ezinstall` is a simple package installer which offers basic features like downloading, checksum verification, tarball
extraction and `install.sh`/`uninstall.sh` script execution.


## Why?
`ezinstall` offers a simple building block which can be used in more complex workflows like automatic deployments etc.

Often it is much easier to just perform a `git clone` with subsequent manual installation than to register your package
in a central package registry, getting it approved, deploying update keys etc.

`ezinstall` attempts to make this manual installation a little bit more generic and secure as it allows not only the
direct installation of conformant git tags but it also offers builtin checksum verification to ensure that nobody has
silently modified the expected tarball (see also [verifiable urls](#verifiable-uris)).


## Package format
Each package is a simple gzip-compressed tarball which contains two scripts:
 - `install.sh` to build and install the package
 - `uninstall.sh` to uninstall the package
  

### Verifiable URLs
Packages are referenced by special URL format `<tarball url...>#sha256=<hex digest...>`, where `hex digest...` is the
hex-encoded SHA2-256 digest of the tarball. This allows the user to pin a specific tarball, which can be useful if you
are downloading from an untrusted source.
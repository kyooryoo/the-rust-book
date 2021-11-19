# Background
This section is a game project for guessing number.
It demonstrates code syntax and project structure.

## Cargo.toml
It is the project configuration file which contains:
* Project name, version, and Rust version.
* Dependencies of other crates besides the standard.
Dependency and its version is defined as:
```
[dependencies]
<crate> = "<x.y.z>"
```
* x.y.z here is the version number such as *0.8.3*
* Cargo will use the latest sub version in building
* It means the latest version of *0.8.x* will be used
* However, any version above *0.9.0* will not be used

## Cargo.lock
Cargo creates this file during the first build.
It contains the version of all dependencies.
It nsures rebuilding with the same dependencies.
Further build will use only the specified versions.
Run `cargo update` to update the dependency versions.

## Code
Some functions return *enum* typed values:
* read_line() returns *Ok* or *Err*
* cmp() returns Ordering enum variants
Use *match* to conditionall react to the response
## Overview
NgSpice is an open-sourced electronics circuit simulated based on the venerable SPICE simulator developed and maintained at UC Berkeley during the 80s and 90s. It is the industry standard: to this day, SPICE simulation models are what vendors provide along their discreet device offerings. 

At the moment, Paprika is just me poking around the NgSpice sharedspice.dll from Rust. 

## Installation
Obtain the appropriate `sharedspice` lib from [here](https://ngspice.sourceforge.io/shared.html). For windows, `sharedspice.dll` from NgSpice38 is included in the repo. `.so` for linux need to be compiled from source, and the binding is not tested.

## Explanation
Compiling `main.rs` produces a simple command line program which passes messages between the user and NgSpice's `command` call. <span style="color:green">stdout</span>, <span style="color:red">stderr</span>, and <span style="color:blue">stats</span> are color coded. If you see <span style="color:magenta">~~something like this~~</span> please open an issue detailing how to reproduce it.

Both `main.rs` and `tests/lib.rs` contains simple examples of how a manager may be implemented. 

## Contribute
Any contribution is welcome. 

## Resources
[NgSpice Beginner Tutorial](https://ngspice.sourceforge.io/ngspice-tutorial.html)

[NgSpice Manual](https://ngspice.sourceforge.io/docs/ngspice-39-manual.pdf)

## Other Bindings
### Rust
[NyanCAD NgSpice bindings](https://github.com/NyanCAD/rust-ngspice)  (Incomplete, no documentations nor code comments. ngspice-sys on crates.io)

[elektron_ngspice](https://github.com/spielhuus/elektron_ngspice/blob/main/src/lib.rs) (Incomplete, no documentations nor code comments. elektron_ngspice on crates.io. Almost identical to NyanCAD above)

### Others
[KiCad NgSpice Binding (cpp)](https://gitlab.com/kicad/code/kicad/-/blob/master/eeschema/sim/ngspice.cpp)

[PySpice NgSpice Binding (Python)](https://github.com/PySpice-org/PySpice/blob/master/PySpice/Spice/NgSpice/Shared.py)

Some existing NgSpice based simulators can be found [here](https://ngspice.sourceforge.io/resources.html)

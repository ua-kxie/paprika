## Overview
NgSpice is an open-sourced electronics circuit simulated based on the venerable SPICE simulator developed and maintained at UC Berkeley during the 80s and 90s. It is the industry standard: to this day, SPICE simulation models are what vendors provide along their discreet device offerings. 

At the moment, Paprika is just me poking around the NgSpice sharedspice.dll from Rust. 

## Explanation
The program in main.rs simply initializes NgSpice and loads the netlist "dcop.cir".
The netlist includes statements to perform the dc operating point analysis, and to output it into a file called "dcop.sim".
Inside, all nets in the circuit is listed, followed by an array of float64 corresponding to the voltages of those nodes. 

## Resources

[NgSpice Beginner Tutorial](https://ngspice.sourceforge.io/ngspice-tutorial.html)

[NgSpice Manual](https://ngspice.sourceforge.io/docs/ngspice-39-manual.pdf)

[KiCad NgSpice Binding (cpp)](https://gitlab.com/kicad/code/kicad/-/blob/master/eeschema/sim/ngspice.cpp)

[PySpice NgSpice Binding (Python)](https://github.com/PySpice-org/PySpice/blob/master/PySpice/Spice/NgSpice/Shared.py)

Some existing NgSpice based simulators can be found [here](https://ngspice.sourceforge.io/resources.html)

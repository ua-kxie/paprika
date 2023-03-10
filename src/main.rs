// with thanks to 
// https://github.com/kmdouglass/rust-libloading-example
// https://users.rust-lang.org/t/callback-based-c-ffi/26583/5

use ::paprika;

struct NgSpiceManager{
}

impl paprika::NgSpiceManager for NgSpiceManager{
    fn cb_send_char(msg: String, id: i32) {
        println!("idiomatic sendchar {}; {};", msg, id)
    }
}


fn main() {
    let ngspice = paprika::NgSpice::new();
    let manager = NgSpiceManager{};
    ngspice.init(manager);
    ngspice.command("source dcop.cir");  // in this case, simulation commands are included inside the netlist and simply sourcing it produces an output.
    ngspice.command("echo hello");
}
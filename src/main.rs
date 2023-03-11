// with thanks to 
// https://github.com/kmdouglass/rust-libloading-example
// https://users.rust-lang.org/t/callback-based-c-ffi/26583/5

use ::paprika;
use paprika::PkVecvaluesall;

struct NgSpiceManager{
    test: Vec<PkVecvaluesall>,
}

impl paprika::NgSpiceManager for NgSpiceManager{
    fn cb_send_char(&mut self, msg: &str, id: i32) {
        println!("idiomatic sendchar {}; {};", msg, id);
    }
    // fn cb_send_init_data() {
    //     todo!();
    // }
    fn cb_send_data(&mut self, pkvecvaluesall: paprika::PkVecvaluesall, count: i32, id: i32) {
        // switch case based on simulation and expected return?
        // println!("idiomatic senddata {:?}; {}; {};", pkvecvaluesall, count, id);
        self.test.push(pkvecvaluesall)
    }
}
fn main() {
    let ngspice = paprika::NgSpice::new();
    let manager = NgSpiceManager{test:Vec::<paprika::PkVecvaluesall>::new()};
    ngspice.init(&manager);
    ngspice.command("source ac.cir");  // in this case, simulation commands are included inside the netlist and simply sourcing it produces an output.
    // ngspice.command("source dcop1.cir");  // results pointer array starts at same address
    // dbg!(&manager.test[0].vecsa[0]);
    ngspice.command("echo hello");
}
// with thanks to 
// https://github.com/kmdouglass/rust-libloading-example
// https://users.rust-lang.org/t/callback-based-c-ffi/26583/5

use ::paprika;
use paprika::{PkVecvaluesall, PkVecinfoall};

struct NgSpiceManager{
    vec_pkvecinfoall: Vec<PkVecinfoall>,
    vec_pkvecvalsall: Vec<PkVecvaluesall>,
    vec_usize: Vec<usize>,
}
impl NgSpiceManager{
    fn new() -> NgSpiceManager{
        NgSpiceManager{
            vec_pkvecinfoall: Vec::<paprika::PkVecinfoall>::new(),
            vec_pkvecvalsall: Vec::<paprika::PkVecvaluesall>::new(),
            vec_usize: Vec::<usize>::new(),
        }
    }
}

impl paprika::NgSpiceManager for NgSpiceManager{
    fn cb_send_char(&mut self, msg: &str, id: i32) {
        println!("idiomatic sendchar {}; {};", msg, id);
    }
    fn cb_send_init_data(&mut self, pkvecinfoall: paprika::PkVecinfoall, id: i32) {
        self.vec_pkvecinfoall.push(pkvecinfoall);
    }
    fn cb_send_data(&mut self, pkvecvaluesall: paprika::PkVecvaluesall, count: i32, id: i32) {
        // switch case based on simulation and expected return?
        self.vec_pkvecvalsall.push(pkvecvaluesall);
    }
}
fn main() {
    let ngspice = paprika::NgSpice::new();
    let mut manager = NgSpiceManager::new();

    ngspice.init(&manager);
    ngspice.command("source ac.cir");  // in this case, simulation commands are included inside the netlist and simply sourcing it produces an output.
    // ngspice.command("source dcop1.cir");  // results pointer array starts at same address
    // ngspice.command("source tran.cir");  // results pointer array starts at same address
    ngspice.command("echo hello");
}
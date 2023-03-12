// with thanks to 
// https://github.com/kmdouglass/rust-libloading-example
// https://users.rust-lang.org/t/callback-based-c-ffi/26583/5

use ::paprika;
use paprika::{PkVecvaluesall, PkVecinfoall};

use colored::Colorize;

struct NgSpiceManager{
    vec_pkvecinfoall: Vec<PkVecinfoall>,
    vec_pkvecvalsall: Vec<PkVecvaluesall>,
}
impl NgSpiceManager{
    fn new() -> NgSpiceManager{
        NgSpiceManager{
            vec_pkvecinfoall: Vec::<paprika::PkVecinfoall>::new(),
            vec_pkvecvalsall: Vec::<paprika::PkVecvaluesall>::new(),
        }
    }
}
#[allow(unused_variables)]
impl paprika::NgSpiceManager for NgSpiceManager{
    fn cb_send_char(&mut self, msg: &str, id: i32) {
        let (token, msgs) = msg.split_once(' ').expect("cb_send_char string split failed");
        let msgc = match token {
            "stdout" => msgs.green(),
            "stderr" => msgs.red(),
            _ => msg.magenta().strikethrough(),
        };
        println!("{}", msgc);
    }
    fn cb_send_stat(&mut self, msg: &str, id: i32) {
        // println!("send_stat {}; {};", msg.blue(), id);
        println!("{}", msg.blue());
    }
    fn cb_ctrldexit(&mut self, status: i32, is_immediate: bool, is_quit: bool, id: i32) {
        println!("ctrldexit {}; {}; {}; {};", status, is_immediate, is_quit, id);
    }
    fn cb_send_init(&mut self, pkvecinfoall: paprika::PkVecinfoall, id: i32) {
        self.vec_pkvecinfoall.push(pkvecinfoall);
    }
    fn cb_send_data(&mut self, pkvecvaluesall: paprika::PkVecvaluesall, count: i32, id: i32) {
        // switch case based on simulation and expected return?
        self.vec_pkvecvalsall.push(pkvecvaluesall);
    }
    fn cb_bgt_state(&mut self, is_fin: bool, id: i32) {
        println!("bgt_state {}; {};", is_fin, id);
    }
}
fn main() {
    let ngspice = paprika::NgSpice::new();
    let manager = NgSpiceManager::new();

    ngspice.init(&manager);
    ngspice.command("source ac.cir");  // in this case, simulation commands are included inside the netlist and simply sourcing it produces an output.
    // ngspice.command("source dcop1.cir");  // results pointer array starts at same address
    // ngspice.command("source tran.cir");  // results pointer array starts at same address
    ngspice.command("echo hello");
}
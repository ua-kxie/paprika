// with thanks to 
// https://github.com/kmdouglass/rust-libloading-example
// https://users.rust-lang.org/t/callback-based-c-ffi/26583/5

use ::paprika;
use paprika::*;

use colored::Colorize;
#[allow(dead_code)]
struct PkSpiceManager{
    lib: PkSpice,
    vec_char: Vec<String>,
    vec_stat: Vec<String>,
    vec_pkvecinfoall: Vec<PkVecinfoall>,
    vec_pkvecvalsall: Vec<PkVecvaluesall>,
}
impl PkSpiceManager {
    fn new() -> PkSpiceManager {
        let manager = PkSpiceManager{ 
            lib: PkSpice::new(), 
            vec_char: Vec::<String>::new(),
            vec_stat: Vec::<String>::new(),
            vec_pkvecinfoall: Vec::<PkVecinfoall>::new(),
            vec_pkvecvalsall: Vec::<PkVecvaluesall>::new(),
        };
        manager.lib.init(&manager);
        manager
    }
}
#[allow(unused_variables)]
impl paprika::PkSpiceManager for PkSpiceManager{
    fn cb_send_char(&mut self, msg: String, id: i32) {
        let (token, msgs) = msg.split_once(' ').expect("cb_send_char string split failed");
        let msgc = match token {
            "stdout" => msgs.green(),
            "stderr" => msgs.red(),
            _ => msg.magenta().strikethrough(),
        };
        println!("{}", msgc);
    }
    fn cb_send_stat(&mut self, msg: String, id: i32) {
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
    let mut manager = PkSpiceManager::new();
    manager.lib.command("source ac.cir");  // in this case, simulation commands are included inside the netlist and simply sourcing it produces an output.
    // manager.lib.command("source dcop1.cir");  // results pointer array starts at same address
    // manager.lib.command("source tran.cir");  // results pointer array starts at same address
    manager.lib.command("echo hello");
}
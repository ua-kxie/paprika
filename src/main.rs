// with thanks to 
// https://github.com/kmdouglass/rust-libloading-example
// https://users.rust-lang.org/t/callback-based-c-ffi/26583/5

use ::paprika;
use paprika::{PkVecvaluesall, PkVecinfoall};

struct NgSpiceManager{
    vec_pkvecinfoall: Vec<PkVecinfoall>,
    vec_pkvecvalsall: Vec<PkVecvaluesall>,
    vec_test: Vec<String>,
}
impl NgSpiceManager{
    fn new() -> NgSpiceManager{
        NgSpiceManager{
            vec_pkvecinfoall: Vec::<paprika::PkVecinfoall>::new(),
            vec_pkvecvalsall: Vec::<paprika::PkVecvaluesall>::new(),
            vec_test: Vec::<String>::new(),
        }
    }
    fn test(&mut self, a: paprika::PkVecinfoall) {
        self.vec_pkvecinfoall.push(a);
    }
}

impl paprika::NgSpiceManager for NgSpiceManager{
    fn cb_send_char(&mut self, msg: &str, id: i32) {
        println!("idiomatic sendchar {}; {};", msg, id);
    }
    fn cb_send_init_data(&mut self, pkvecinfoall: paprika::PkVecinfoall, count: i32, id: i32) {
        // self.vec_pkvecinfoall.push(pkvecinfoall);
        // self.vec_test.push("".to_string());
        // self.vec_pkvecinfoall.push(paprika::PkVecinfoall{
        //     name: "".to_string(),
        //     title: "".to_string(),
        //     date: "".to_string(),
        //     stype: "".to_string(),
        //     count: 1,
        //     // vecs: Vec::<paprika::PkVecinfo>::with_capacity(0),
        // });
        let a = paprika::PkVecinfoall{
            count: 1,
        };
        println!("{:p}", &a);
        self.vec_pkvecinfoall.push(a);
    }
    fn cb_send_data(&mut self, pkvecvaluesall: paprika::PkVecvaluesall, count: i32, id: i32) {
        // switch case based on simulation and expected return?
        self.vec_pkvecvalsall.push(pkvecvaluesall);
    }
}
fn main() {
    let ngspice = paprika::NgSpice::new();
    let mut manager = NgSpiceManager::new();

    let a = paprika::PkVecinfoall{
        count: 1,
    };
    manager.test(a);

    ngspice.init(&manager);
    // ngspice.command("source ac.cir");  // in this case, simulation commands are included inside the netlist and simply sourcing it produces an output.
    ngspice.command("source dcop1.cir");  // results pointer array starts at same address
    // ngspice.command("source tran.cir");  // results pointer array starts at same address
    // ngspice.command("echo hello");
}
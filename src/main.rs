// with thanks to 
// https://github.com/kmdouglass/rust-libloading-example
// https://users.rust-lang.org/t/callback-based-c-ffi/26583/5

use ::paprika;

struct NgSpiceManager{
    test: String,
}

impl paprika::NgSpiceManager for NgSpiceManager{
    fn cb_send_char(&mut self, msg: &str, id: i32) {
        println!("idiomatic sendchar {}; {};", msg, id);
    }
    fn cb_send_data(&mut self, ptr: *const char, count: i32, id: i32) {
        println!("idiomatic senddata {:p}; {}; {};", ptr, count, id);
        unsafe{
            let a = *(ptr as *const paprika::NgVecvaluesall);
            println!("{:p}", a.vecsa as * const usize);
        }
    }
}
fn main() {
    let ngspice = paprika::NgSpice::new();
    let manager = NgSpiceManager{test:"".to_string()};
    ngspice.init(&manager);
    // ngspice.command("source ac.cir");  // in this case, simulation commands are included inside the netlist and simply sourcing it produces an output.
    ngspice.command("source dcop1.cir");  // results pointer array starts at same address
    ngspice.command("echo hello");
}
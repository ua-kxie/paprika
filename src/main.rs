// with thanks to https://github.com/kmdouglass/rust-libloading-example

extern crate libloading;
extern crate libc;

#[cfg(unix)]
use libloading::os::unix::Symbol as RawSymbol;
#[cfg(windows)]
use libloading::os::windows::Symbol as RawSymbol;
use libloading::{Library, Symbol};
use libc::*;

extern fn cb_send_char(msg: *const c_char, id: c_int, user: c_void) -> c_int{
    let cstr = unsafe { std::ffi::CStr::from_ptr(msg)};
    println!("sendchar: {:?}; {}; {:?}", cstr, id, user);
    return 0;
}

extern fn cb_send_stat(msg: *const c_char, id: c_int, user: c_void) -> c_int{
    let cstr = unsafe { std::ffi::CStr::from_ptr(msg)};
    println!("sendstat: {:?}; {}; {:?}", cstr, id, user);
    return 0;
}
extern fn cb_controlled_exit(status: c_int, immediate: bool, exit_on_quit: bool, id: c_int, user: c_void) -> c_int{
    println!("ctrldexit: {}; {}; {}; {}; {:?}", status, immediate, exit_on_quit, id, user);
    return 0;
}
extern fn cb_bgthread_running(finished: bool, id: c_int, user: c_void) -> c_int{
    println!("bgrunning: {}; {}; {:?}", finished, id, user);
    return 0;
}

type NgSpiceInit = fn(
    extern fn(*const c_char, c_int, c_void) -> c_int, 
    extern fn(*const c_char, c_int, c_void) -> c_int, 
    extern fn(c_int, bool, bool, c_int, c_void) -> c_int, 
    *const c_char, 
    *const c_char, 
    extern fn(bool, c_int, c_void) -> c_int, 
    *const c_char, 
) -> bool;
type NgSpiceCommand = fn(std::ffi::CString) -> bool;

struct VTableV0 {
    init: RawSymbol<NgSpiceInit>,
    command: RawSymbol<NgSpiceCommand>
}

impl VTableV0 {
    unsafe fn new(library: &Library) -> VTableV0 {
        println!("Loading API version 0...");
        let init: Symbol<NgSpiceInit> = library.get(b"ngSpice_Init\0").unwrap();
        let init = init.into_raw();
        let command: Symbol<NgSpiceCommand> = library.get(b"ngSpice_Command\0").unwrap();
        let command = command.into_raw();

        VTableV0 {
            init: init,
            command: command,
        }
    }
}

struct Plugin {
    #[allow(dead_code)]
    library: Library,
    vtable: VTableV0
}

impl Plugin {
    unsafe fn new(library_name: &str) -> Plugin {
        let library = Library::new(library_name).unwrap();

        let vtable = VTableV0::new(&library);

        Plugin {
            library: library,
            vtable: vtable,
        }
    }

    fn command(self, command: &str) -> bool {
        (self.vtable.command)(std::ffi::CString::new(command).unwrap())
    }
}

fn main() {
    let ngspice = unsafe { Plugin::new("src/ngspice.dll") };
    (ngspice.vtable.init)(cb_send_char, cb_send_stat, cb_controlled_exit, std::ptr::null(), std::ptr::null(), cb_bgthread_running, std::ptr::null());
    ngspice.command("source dcop.cir");  // in this case, simulation commands are included inside the netlist and simply sourcing it produces an output.
}

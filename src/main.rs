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
extern fn cb_send_data(pvecvaluesall: *const NgVecvaluesall, count: c_int, id: c_int, user: c_void) -> c_int{
    println!("senddata: {}; {}; {:?}; {:?};", count, id, user, pvecvaluesall);
    unsafe{(*pvecvaluesall).debug();}  // simulation results are returned via this callback. 
    return 0;
}
extern fn cb_send_init_data(pvecinfoall: *const i8, count: c_int, id: c_int, user: c_void) -> c_int{
    println!("sendinitdata: {}; {}; {:?};", count, id, user);
    return 0;
}
extern fn cb_bgthread_running(finished: bool, id: c_int, user: c_void) -> c_int{
    println!("bgrunning: {}; {}; {:?}", finished, id, user);
    return 0;
}

#[derive(Copy, Clone)]
#[repr(C)]
struct NgVecvalues {
    name: *const c_char,
    creal: c_double,
    cimag: c_double,
    is_scale: bool,
    is_complex: bool
}
impl NgVecvalues {
    fn debug(self) {
        println!("{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n", unsafe { std::ffi::CStr::from_ptr(self.name)}, self.creal, self.cimag, self.is_scale, self.is_complex);
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
struct NgVecvaluesall {
    count: c_int,
    index: c_int,
    vecsa: *const *const NgVecvalues,
}
impl NgVecvaluesall {
    fn debug(self) {
        let vec1 = unsafe{std::slice::from_raw_parts(self.vecsa, self.count as usize)};
        for s in vec1.iter() {
            let a = unsafe{ &*(*s) };
            a.debug();
        }
    }
}

type NgSpiceInit = fn(
    extern fn(*const c_char, c_int, c_void) -> c_int, 
    extern fn(*const c_char, c_int, c_void) -> c_int, 
    extern fn(c_int, bool, bool, c_int, c_void) -> c_int, 
    extern fn(*const NgVecvaluesall, c_int, c_int, c_void) -> c_int, 
    extern fn(*const i8, c_int, c_int, c_void) -> c_int,  
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
    (ngspice.vtable.init)(cb_send_char, cb_send_stat, cb_controlled_exit, cb_send_data, cb_send_init_data, cb_bgthread_running, std::ptr::null());
    ngspice.command("source dcop.cir");  // in this case, simulation commands are included inside the netlist and simply sourcing it produces an output.
}

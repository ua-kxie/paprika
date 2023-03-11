extern crate libloading;
extern crate libc;

#[cfg(unix)]
use libloading::os::unix::Symbol as RawSymbol;
#[cfg(windows)]
use libloading::os::windows::Symbol as RawSymbol;
use libloading::{Library, Symbol};
use libc::*;

extern fn cbw_send_char<T>(msg: *const c_char, id: c_int, user: *const c_void) -> c_int where T: NgSpiceManager{
    unsafe{
        // &mut *(user as *mut T)
        // let result = std::ffi::CStr::from_ptr(msg).to_str();
        // let mut msgstr = "";
        // match result {
        //     Ok(s)=>{
        //         let msgstr = s;
        //     },
        //     Err(msg)=>{
        //        println!("Error msg is {}", msg);
        //     }
        //  }
        <T as NgSpiceManager>::cb_send_char(&mut *(user as *mut T), std::ffi::CStr::from_ptr(msg).to_str().unwrap(), id);
    }
    return 0;
}
extern fn cbw_send_stat<T>(msg: *const c_char, id: c_int, user: *const c_void) -> c_int where T: NgSpiceManager{
    let cstr = unsafe { std::ffi::CStr::from_ptr(msg)};
    println!("sendstat: {:?}; {}; {:?}", cstr, id, user);
    return 0;
}
extern fn cbw_controlled_exit<T>(status: c_int, immediate: bool, exit_on_quit: bool, id: c_int, user: *const c_void) -> c_int where T: NgSpiceManager{
    println!("ctrldexit: {}; {}; {}; {}; {:?}", status, immediate, exit_on_quit, id, user);
    return 0;
}
extern fn cbw_send_data<T>(pvecvaluesall: *const NgVecvaluesall, count: c_int, id: c_int, user: *const c_void) -> c_int where T: NgSpiceManager{
    // do not free pvecvaluesall or pvecvalues - the memory is reused by ngspice
    unsafe{
        let vecvals_slice = std::slice::from_raw_parts((*pvecvaluesall).vecsa, (*pvecvaluesall).count as usize);
        // create vec containing 'count' number of PkVecvalues
        let mut pkvecvalues = Vec::<PkVecvalues>::with_capacity((*pvecvaluesall).count as usize);
        // for item in vecvals_slice:
        for item in vecvals_slice.iter() {
            // create native PkVecvalues and store into vec
            pkvecvalues.push((*(*item)).to_pk());
        }
        // create native PkVecvaluesall
        let pkvecinfoall = PkVecvaluesall{
            count:(*pvecvaluesall).count, 
            index: (*pvecvaluesall).index, 
            vecsa: pkvecvalues
        };
        
        // call native callback
        <T as NgSpiceManager>::cb_send_data(&mut *(user as *mut T), pkvecinfoall, count, id);
    }
    return 0;
}
extern fn cbw_send_init_data<T>(pvecinfoall: *const NgVecinfoall, count: c_int, id: c_int, user: *const c_void) -> c_int where T: NgSpiceManager{
    unsafe {
        // let vecinfos_slice = std::slice::from_raw_parts((*pvecinfoall).vecs, (*pvecinfoall).veccount as usize);
        // // create vec containing 'count' number of PkVecvalues
        // let mut pkvecinfos = Vec::<PkVecinfo>::with_capacity((*pvecinfoall).veccount as usize);
        // // for item in vecvals_slice:
        // for item in vecinfos_slice.iter() {
        //     // create native PkVecvalues and store into vec
        //     pkvecinfos.push((*(*item)).to_pk());
        // }
        // // create native PkVecvaluesall
        // let pkvecinfoall = PkVecinfoall{
        //     name: std::ffi::CStr::from_ptr((*pvecinfoall).name).to_str().unwrap().to_string(),
        //     title: std::ffi::CStr::from_ptr((*pvecinfoall).title).to_str().unwrap().to_string(),
        //     date: std::ffi::CStr::from_ptr((*pvecinfoall).date).to_str().unwrap().to_string(),
        //     stype: std::ffi::CStr::from_ptr((*pvecinfoall).type_).to_str().unwrap().to_string(),
        //     count: (*pvecinfoall).veccount,
        //     vecs: pkvecinfos,
        // };
        // // println!("sendinitdata: {:?}; {}; {}; {:?};", pkvecinfoall, count, id, user);
        // <T as NgSpiceManager>::cb_send_init_data(&mut *(user as *mut T), pkvecinfoall, count, id);
        let pkvecinfoall = PkVecinfoall{
            count: 1,
        };
        <T as NgSpiceManager>::cb_send_init_data(&mut *(user as *mut T), pkvecinfoall, count, id);
    }
    return 0;
}
extern fn cbw_bgthread_running<T>(finished: bool, id: c_int, user: *const c_void) -> c_int where T: NgSpiceManager{
    println!("bgrunning: {}; {}; {:?}", finished, id, user);
    return 0;
}
#[derive(Copy, Clone, Debug)]
#[repr(C)]
struct NgVecinfo {
    number: c_int,
    vecname: *const c_char,
    is_real: bool,
    pdvec: *const c_void,  // not elaborated in the docs - not sure if intended for use
    pdvecscale: *const c_void,  // not elaborated in the docs - not sure if intended for use
}
impl NgVecinfo {
    fn to_pk(&self) -> PkVecinfo {
        unsafe {
            PkVecinfo{
                number: self.number,
                name: std::ffi::CStr::from_ptr(self.vecname).to_str().unwrap().to_string(),
                is_real: self.is_real,
                pdvec: self.pdvec as usize,
                pdvecscale: self.pdvecscale as usize,
            }
        }
    }
}
#[derive(Clone, Debug)]
pub struct PkVecinfo {
    pub number: i32,
    pub name: String,
    pub is_real: bool,
    pdvec: usize,  // not elaborated in the docs - not sure if intended for use
    pdvecscale: usize,  // not elaborated in the docs - not sure if intended for use
}
#[derive(Copy, Clone, Debug)]
#[repr(C)]
struct NgVecinfoall {
    name: *const c_char,
    title: *const c_char,
    date: *const c_char,
    type_: *const c_char,
    veccount: c_int,
    vecs: *const *const NgVecinfo,
}

#[derive(Clone, Debug)]
pub struct PkVecinfoall{
    // pub name: String,
    // pub title: String,
    // pub date: String,
    // pub stype: String,
    pub count: i32,
    // pub vecs: Vec<PkVecinfo>,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
struct NgVecvalues {
    name: *const c_char,
    creal: c_double,
    cimag: c_double,
    is_scale: bool,
    is_complex: bool
}
impl NgVecvalues {
    fn debug(&self) {
        println!("{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n", unsafe { std::ffi::CStr::from_ptr(self.name)}, self.creal, self.cimag, self.is_scale, self.is_complex);
    }
    fn to_pk(&self) -> PkVecvalues {
        unsafe {
            PkVecvalues{
                name: std::ffi::CStr::from_ptr(self.name).to_owned().into_string().unwrap(),
                creal: self.creal,
                cimag: self.cimag,
                is_scale: self.is_scale,
                is_complex: self.is_complex,
            }
        }
    }
}

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct PkVecvalues {
    name: String,
    creal: f64,
    cimag: f64,
    is_scale: bool,
    is_complex: bool,
}

#[derive(Copy, Clone)]
#[repr(C)]
struct NgVecvaluesall {
    count: c_int,
    index: c_int,
    vecsa: *const *const NgVecvalues,
}
#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct PkVecvaluesall{
    pub count: i32,
    pub index: i32,
    pub vecsa: Vec<PkVecvalues>,
}

type NgSpiceInit = extern fn(
    extern fn(*const c_char, c_int, *const c_void) -> c_int, 
    extern fn(*const c_char, c_int, *const c_void) -> c_int, 
    extern fn(c_int, bool, bool, c_int, *const c_void) -> c_int, 
    extern fn(*const NgVecvaluesall, c_int, c_int, *const c_void) -> c_int, 
    extern fn(*const NgVecinfoall, c_int, c_int, *const c_void) -> c_int,  
    extern fn(bool, c_int, *const c_void) -> c_int, 
    *const c_char, 
) -> bool;
type NgSpiceCommand = extern fn(*const c_char) -> bool;

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

pub trait NgSpiceManager {
    fn cb_send_char(&mut self, msg: &str, id: i32);
    fn cb_send_init_data(&mut self, pkvecinfoall: PkVecinfoall, count: i32, id: i32);
    fn cb_send_data(&mut self, pkvecvaluesall: PkVecvaluesall, count: i32, id: i32);
}

pub struct NgSpice {
    #[allow(dead_code)]
    library: Library,
    api: VTableV0
}

impl Default for NgSpice {
    fn default() -> Self {
    Self::new()
    }
}

impl NgSpice {
    pub fn new() -> NgSpice {
        unsafe {
            let library = Library::new("src/ngspice.dll").unwrap();

            let vtable = VTableV0::new(&library);
            NgSpice {
                library: library,
                api: vtable,
            }
        }
    }

    pub fn init<T>(&self, manager: &T) -> bool where T: NgSpiceManager {
        unsafe{
            (self.api.init)(
                cbw_send_char::<T>, 
                cbw_send_stat::<T>, 
                cbw_controlled_exit::<T>, 
                cbw_send_data::<T>, 
                cbw_send_init_data::<T>, 
                cbw_bgthread_running::<T>, 
                std::mem::transmute(std::ptr::addr_of!(*manager))
            )
        }
    }

    pub fn command(&self, command: &str) -> bool {
        let cmdstr = std::ffi::CString::new(command).unwrap();
        (self.api.command)(cmdstr.as_ptr())
    }
}
use paprika::*;
struct PkSpiceManager{
    lib: PkSpice,
    vec_char: Vec<String>,
    vec_stat: Vec<String>,
    vec_pkvecinfoall: Vec<PkVecinfoall>,
    vec_pkvecvalsall: Vec<PkVecvaluesall>,
}
#[allow(unused_variables)]
impl paprika::PkSpiceManager for PkSpiceManager {
    fn cb_send_char(&mut self, msg: String, id: i32) {
        self.vec_char.push(msg);
    }
    fn cb_send_stat(&mut self, msg: String, id: i32) {
        self.vec_stat.push(msg);
    }
    fn cb_ctrldexit(&mut self, status: i32, is_immediate: bool, is_quit: bool, id: i32) {
        println!("ctrldexit {}; {}; {}; {};", status, is_immediate, is_quit, id);
    }
    fn cb_send_init(&mut self, pkvecinfoall: paprika::PkVecinfoall, id: i32) {
        self.vec_pkvecinfoall.push(pkvecinfoall);
    }
    fn cb_send_data(&mut self, pkvecvaluesall: paprika::PkVecvaluesall, count: i32, id: i32) {
        self.vec_pkvecvalsall.push(pkvecvaluesall);
    }
    fn cb_bgt_state(&mut self, is_fin: bool, id: i32) {
        println!("bgt_state {}; {};", is_fin, id);
    }
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

#[test]
fn test_cmd_echo() {
    let mut manager = PkSpiceManager::new();
    manager.lib.command("echo hello");
    assert_eq!(manager.vec_char.pop().unwrap(), "stdout hello");
}
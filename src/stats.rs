use sysinfo::System;

#[derive(Debug, Clone)]
pub struct Stats {
    used_memory: u64,
    total_memory: u64,
    cpu_cores: usize
}
impl Stats {
    pub(crate) fn new() -> Self {
        Self { used_memory: todo!(), total_memory: todo!(), cpu_cores: todo!() }
    }
}

pub fn get_system_info() -> Stats {
    let mut sys = System::new_all();
    sys.refresh_all();
    println!("deez nuts");
    println!("memory : {}/{} bytes", sys.used_memory(), sys.total_memory());
    println!("cpuzzz: {} cores", sys.cpus().len());
    
    Stats {
        used_memory: sys.used_memory(),
        total_memory: sys.total_memory(),
        cpu_cores: sys.cpus().len()
    }

}

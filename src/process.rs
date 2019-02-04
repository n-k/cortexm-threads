#[repr(C)]
pub struct Process {
    pub id: u8,
    pub stack: [u8],
}

#[repr(C)]
pub struct CPUState {
    pub saved_registers: [usize; 8],
    pub psp: u32,
}

impl Process {
    pub fn switch_to() {
    }

    pub fn switch_from() {}
}

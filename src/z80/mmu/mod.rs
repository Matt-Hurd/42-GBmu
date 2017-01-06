pub struct MMU {

}

impl MMU {
    // Read 8-bit byte at addr
    pub fn rb(&mut self, addr: u16) -> u8 {
        0
    }

    // Read 16-bit word at addr
    pub fn rw(&mut self, addr: u16) -> u16 {
        0
    }

    // Write 8-bit val at addr
    pub fn wb(&mut self, addr: u16, val: u8) {

    }

    // Read 16-bit val at addr
    pub fn ww(&mut self, addr: u16, val: u16) {

    }
}

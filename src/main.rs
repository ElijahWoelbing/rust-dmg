mod cpu;
mod mmu;
fn main(){
    let mut cpu = cpu::CPU::new();
    cpu.execute_opcode();
}

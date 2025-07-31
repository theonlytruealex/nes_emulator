mod constants;
mod cpu;
use crate::cpu::CPU;
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.reg_a, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xaa, 0x00]);
        cpu.reset();
        cpu.reg_a = 10;
        cpu.run();

        assert_eq!(cpu.reg_x, 10)
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.reg_x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xe8, 0xe8, 0x00]);
        cpu.reset();
        cpu.reg_x = 0xff;
        cpu.run();

        assert_eq!(cpu.reg_x, 1)
    }

    #[test]
    fn test_lda_from_memory() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x55);

        cpu.load_and_run(vec![0xa5, 0x10, 0x00]);

        assert_eq!(cpu.reg_a, 0x55);
    }

    #[test]
    fn test_adc_basic() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x55);

        cpu.load_and_run(vec![0xa5, 0x10, 0x69, 0x10, 0x00]);

        assert_eq!(cpu.reg_a, 0x65);
    }

    #[test]
    fn test_adc_from_memory() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x12);

        cpu.load_and_run(vec![0xa5, 0x10, 0x65, 0x10, 0x00]);

        assert_eq!(cpu.reg_a, 0x24);
    }
    #[test]
    fn test_and_basic() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x55);

        cpu.load_and_run(vec![0xa5, 0x10, 0x29, 0x11, 0x00]);

        assert_eq!(cpu.reg_a, (0x55 & 0x11));
    }

    #[test]
    fn test_and_from_memory() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x56);

        cpu.load_and_run(vec![0xa5, 0x10, 0x25, 0x10, 0x00]);

        assert_eq!(cpu.reg_a, (0x56 & 0x56));
    }

    #[test]
    fn test_asl() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x10, 0x0a, 0x00]);

        assert_eq!(cpu.reg_a, (0x20));
        cpu.load_and_run(vec![0xa9, 0xff, 0x0a, 0x00]);

        assert_eq!(cpu.reg_a, (0xfe));
    }

    #[test]
    fn test_asl_from_memory() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x10);
    
        cpu.load_and_run(vec![0x06, 0x10, 0x00]);
    
        let value = cpu.mem_read(0x10);
        assert_eq!(value, 0x20);
    }
}

pub struct CPU {
    pub accumulator: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub stack_pointer: u8,
    pub program_counter: u16,
    pub status_flags: u8,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            accumulator: 0,
            register_x: 0,
            register_y: 0,
            stack_pointer: 0,
            program_counter: 0,
            status_flags: 0,
        }
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        self.program_counter = 0;

        loop {
            // Retrieve opcode
            let opcode = program[self.program_counter as usize];

            // Increment program counter
            self.program_counter += 1;

            // Parse opcode
            match opcode {
                0x00 => {
                    return;
                },
                0x0A => {
                    self.accumulator = self.asl(self.accumulator);
                },
                0x18 => {
                    self.clc();
                },
                0x29 => {
                    let parameter = program[self.program_counter as usize];

                    self.program_counter += 1;
                    self.and(parameter);
                },
                0x30 => {
                    let parameter = program[self.program_counter as usize];

                    self.program_counter += 1;
                    self.bmi(parameter);
                },
                0x50 => {
                    let parameter = program[self.program_counter as usize];

                    self.program_counter += 1;
                    self.bvc(parameter);
                },
                0x90 => {
                    let parameter = program[self.program_counter as usize];

                    self.program_counter += 1;
                    self.bcc(parameter);
                },
                0xA9 => {
                    let parameter = program[self.program_counter as usize];

                    self.program_counter += 1;
                    self.accumulator = parameter;

                    self.lda(parameter);
                },
                0xAA => {
                    self.tax();
                },
                0xB0 => {
                    let parameter = program[self.program_counter as usize];

                    self.program_counter += 1;
                    self.bcs(parameter);
                },
                0xD0 => {
                    let parameter = program[self.program_counter as usize];

                    self.program_counter += 1;
                    self.bne(parameter);
                },
                0xF0 => {
                    let parameter = program[self.program_counter as usize];

                    self.program_counter += 1;
                    self.beq(parameter);
                },
                _ => panic!("Unknown opcode: {}", opcode),
            }
        }
    }

    pub fn asl(&mut self, parameter: u8) -> u8 {
        let result = parameter << 1;

        if result & 0b00000001 == 0b00000001 {
            // Set carry flag
            self.status_flags |= 0b00000001;
        } else {
            // Clear carry flag
            self.status_flags &= 0b11111110;
        }

        if result == 0 {
            // Set zero flag
            self.status_flags |= 0b00000010;
        } else {
            // Clear zero flag
            self.status_flags &= 0b11111101;
        }

        if result & 0b10000000 == 0b10000000 {
            // Set negative flag
            self.status_flags |= 0b10000000;
        } else {
            // Clear negative flag
            self.status_flags &= 0b01111111;
        }

        result
    }

    pub fn and(&mut self, parameter: u8) {
        self.accumulator &= parameter;

        if self.accumulator == 0 {
            // Set zero flag
            self.status_flags |= 0b00000010;
        } else {
            // Clear zero flag
            self.status_flags &= 0b11111101;
        }

        if self.accumulator & 0b10000000 == 0b10000000 {
            // Set negative flag
            self.status_flags |= 0b10000000;
        } else {
            // Clear negative flag
            self.status_flags &= 0b01111111;
        }
    }

    pub fn bcc(&mut self, parameter: u8) {
        if self.status_flags & 0b00000001 == 0b00000000 {
            self.program_counter += parameter as u16;
        }
    }

    pub fn bcs(&mut self, parameter: u8) {
        if self.status_flags & 0b00000001 == 0b00000001 {
            self.program_counter += parameter as u16;
        }
    }

    pub fn beq(&mut self, parameter: u8) {
        if self.status_flags & 0b00000010 == 0b00000010 {
            self.program_counter += parameter as u16;
        }
    }

    pub fn bit(&mut self, parameter: u8) {
        if self.accumulator & parameter == 0 {
            // Set zero flag
            self.status_flags |= 0b00000010;
        } else {
            // Clear zero flag
            self.status_flags &= 0b11111101;
        }

        if parameter & 0b01000000 == 0b01000000 {
            // Set overflow flag
            self.status_flags |= 0b01000000;
        } else {
            // Clear overflow flag
            self.status_flags &= 0b10111111;
        }

        if parameter & 0b10000000 == 0b10000000 {
            // Set negative flag
            self.status_flags |= 0b10000000;
        } else {
            // Clear negative flag
            self.status_flags &= 0b01111111;
        }
    }

    pub fn bmi(&mut self, parameter: u8) {
        if self.status_flags & 0b10000000 == 0b10000000 {
            self.program_counter += parameter as u16;
        }
    }

    pub fn bne(&mut self, parameter: u8) {
        if self.status_flags & 0b00000010 == 0b00000000 {
            self.program_counter += parameter as u16;
        }
    }
    
    pub fn bpl(&mut self, parameter: u8) {
        if self.status_flags & 0b10000000 == 0b00000000 {
            self.program_counter += parameter as u16;
        }
    }

    pub fn bvc(&mut self, parameter: u8) {
        if self.status_flags & 0b01000000 == 0b00000000 {
            self.program_counter += parameter as u16;
        }
    }

    pub fn clc(&mut self) {
        // Clear carry flag
        self.status_flags &= 0b11111110;
    }

    pub fn lda(&mut self, parameter: u8) {
        self.accumulator = parameter;

        if self.accumulator == 0 {
            // Set zero flag
            self.status_flags |= 0b00000010;
        } else {
            // Clear zero flag
            self.status_flags &= 0b11111101;
        }

        if self.accumulator & 0b10000000 == 0b10000000 {
            // Set negative flag
            self.status_flags |= 0b10000000;
        } else {
            // Clear negative flag
            self.status_flags &= 0b01111111;
        }
    }

    pub fn tax(&mut self) {
        self.register_x = self.accumulator;

        if self.register_x == 0 {
            // Set zero flag
            self.status_flags |= 0b00000010;
        } else {
            // Clear zero flag
            self.status_flags &= 0b11111101;
        }

        if self.register_x & 0b10000000 == 0b10000000 {
            // Set negative flag
            self.status_flags |= 0b10000000;
        } else {
            // Clear negative flag
            self.status_flags &= 0b01111111;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_and_immediate() {
        let mut cpu = CPU::new();

        cpu.interpret(vec![0xA9, 0x05, 0x29, 0x03, 0x00]);

        assert_eq!(cpu.accumulator, 0x01);
        assert_eq!(cpu.status_flags, 0b00000000);
    }

    #[test]
    fn test_asl_accumulator() {
        let mut cpu = CPU::new();

        cpu.interpret(vec![0xA9, 0x05, 0x0A, 0x00]);

        assert_eq!(cpu.accumulator, 0x0A);
        assert_eq!(cpu.status_flags, 0b00000000);
    }

    #[test]
    fn test_bcc() {
        let mut cpu = CPU::new();

        cpu.interpret(vec![0x90, 0x02, 0xA9, 0x05, 0x00]);

        assert_eq!(cpu.accumulator, 0x05);
        assert_eq!(cpu.status_flags, 0b00000000);
    }

    #[test]
    fn test_lda_immediate() {
        let mut cpu = CPU::new();

        cpu.interpret(vec![0xA9, 0x05, 0x00]);

        assert_eq!(cpu.accumulator, 0x05);
        assert_eq!(cpu.status_flags, 0b00000000);
    }

    #[test]
    fn test_lda_immediate_zero() {
        let mut cpu = CPU::new();

        cpu.interpret(vec![0xA9, 0x00, 0x00]);

        assert_eq!(cpu.accumulator, 0x00);
        assert_eq!(cpu.status_flags, 0b00000010); // Assert zero flag is set
    }

    #[test]
    fn test_lda_immediate_negative() {
        let mut cpu = CPU::new();

        cpu.interpret(vec![0xA9, 0xFF, 0x00]);

        assert_eq!(cpu.accumulator, 0xFF);
        assert_eq!(cpu.status_flags, 0b10000000); // Assert negative flag is set
    }

    #[test]
    fn test_tax() {
        let mut cpu = CPU::new();

        cpu.interpret(vec![0xA9, 0x05, 0xAA, 0x00]);

        assert_eq!(cpu.accumulator, 0x05);
        assert_eq!(cpu.register_x, 0x05);
        assert_eq!(cpu.status_flags, 0b00000000);
    }

    #[test]
    fn test_tax_zero() {
        let mut cpu = CPU::new();

        cpu.interpret(vec![0xA9, 0x00, 0xAA, 0x00]);

        assert_eq!(cpu.accumulator, 0x00);
        assert_eq!(cpu.register_x, 0x00);
        assert_eq!(cpu.status_flags, 0b00000010); // Assert zero flag is set
    }

    #[test]
    fn test_tax_negative() {
        let mut cpu = CPU::new();

        cpu.interpret(vec![0xA9, 0xFF, 0xAA, 0x00]);

        assert_eq!(cpu.accumulator, 0xFF);
        assert_eq!(cpu.register_x, 0xFF);
        assert_eq!(cpu.status_flags, 0b10000000); // Assert negative flag is set
    }
}
use crate::gb::*;
use crate::gb::cpu::Cpu;
use crate::gb::joypad::JoypadInterrupt;
use piston_window::*;
use crate::frontend::*;

pub struct Emu {
    pub gb: Gb,
    pub frontend: Box<Frontend>,
}

impl Emu {
    pub fn run_loop(&mut self, skip_bootrom: bool) {
        let gb = &mut self.gb;
        let mut breakpoints: Vec<u16> = vec![];
        let mut is_debug = false;

        if skip_bootrom {
            gb.cpu.pc = 0x100;
            init_io_registers(&mut gb.cpu);
        }

        let mut last_frame_nanos = std::time::Instant::now();
        loop {
            let should_redraw = gb.run_machine_cycle(is_debug);
            if breakpoints.contains(&gb.cpu.pc) {
                is_debug = true;
            }
            if is_debug && !gb.cpu.is_busy() {
                is_debug = do_debug_stuff(&gb.cpu, &mut breakpoints);
            }

            if should_redraw {
                let frame_interval_nanos = std::time::Duration::from_nanos(16_742_005);
                let current_time = std::time::Instant::now();
                let d_t = current_time - last_frame_nanos;
                if frame_interval_nanos > d_t {
                    let sleep_dur = frame_interval_nanos - d_t;
                    std::thread::sleep(sleep_dur);
                }
                self.frontend.render(gb);

                last_frame_nanos = std::time::Instant::now();
            }
        }
    }
}

fn do_debug_stuff(cpu: &Cpu, breakpoints: &mut Vec<u16>) -> bool {
    print_registers(&cpu);
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let continue_debugging = match line.trim() {
        "q" => false,
        l => {
            if let Ok(addr) = u16::from_str_radix(l, 16) {
                *breakpoints = vec![addr];
                false
            } else {
                true
            }
        }
    };
    continue_debugging
}

fn print_registers(cpu: &Cpu) {
    eprintln!();
    eprintln!("af: {:02X}{:02X} ", cpu.a, cpu.f);
    eprintln!("Z = {}", cpu.get_z());
    eprintln!("bc: {:02X}{:02X}", cpu.b, cpu.c);
    eprintln!("de: {:02X}{:02X}", cpu.d, cpu.e);
    eprintln!("hl: {:02X}{:02X}", cpu.h, cpu.l);
    eprintln!("sp: {:04X}", cpu.sp);
    eprintln!("pc: {:04X}", cpu.pc);
    eprintln!("FF42(SC_Y): {:02X}", cpu.mmu.read_byte(0xFF42));
    eprintln!("FF44: {:02X}", cpu.mmu.read_byte(0xFF44));
}

fn print_io_registers(cpu: &Cpu) {
    eprintln!("[$FF04] = {:02x} ($AB) ; DIV ", cpu.mmu.read_byte(0xFF04));
    eprintln!("[$FF05] = {:02x} ($00) ; TIMA", cpu.mmu.read_byte(0xFF05));
    eprintln!("[$FF06] = {:02x} ($00) ; TMA ", cpu.mmu.read_byte(0xFF06));
    eprintln!("[$FF07] = {:02x} ($00) ; TAC ", cpu.mmu.read_byte(0xFF07));
    eprintln!("[$FF10] = {:02x} ($80) ; NR10", cpu.mmu.read_byte(0xFF10));
    eprintln!("[$FF11] = {:02x} ($BF) ; NR11", cpu.mmu.read_byte(0xFF11));
    eprintln!("[$FF12] = {:02x} ($F3) ; NR12", cpu.mmu.read_byte(0xFF12));
    eprintln!("[$FF14] = {:02x} ($BF) ; NR14", cpu.mmu.read_byte(0xFF14));
    eprintln!("[$FF16] = {:02x} ($3F) ; NR21", cpu.mmu.read_byte(0xFF16));
    eprintln!("[$FF17] = {:02x} ($00) ; NR22", cpu.mmu.read_byte(0xFF17));
    eprintln!("[$FF19] = {:02x} ($BF) ; NR24", cpu.mmu.read_byte(0xFF19));
    eprintln!("[$FF1A] = {:02x} ($7F) ; NR30", cpu.mmu.read_byte(0xFF1A));
    eprintln!("[$FF1B] = {:02x} ($FF) ; NR31", cpu.mmu.read_byte(0xFF1B));
    eprintln!("[$FF1C] = {:02x} ($9F) ; NR32", cpu.mmu.read_byte(0xFF1C));
    eprintln!("[$FF1E] = {:02x} ($BF) ; NR33", cpu.mmu.read_byte(0xFF1E));
    eprintln!("[$FF20] = {:02x} ($FF) ; NR41", cpu.mmu.read_byte(0xFF20));
    eprintln!("[$FF21] = {:02x} ($00) ; NR42", cpu.mmu.read_byte(0xFF21));
    eprintln!("[$FF22] = {:02x} ($00) ; NR43", cpu.mmu.read_byte(0xFF22));
    eprintln!("[$FF23] = {:02x} ($BF) ; NR30", cpu.mmu.read_byte(0xFF23));
    eprintln!("[$FF24] = {:02x} ($77) ; NR50", cpu.mmu.read_byte(0xFF24));
    eprintln!("[$FF25] = {:02x} ($F3) ; NR51", cpu.mmu.read_byte(0xFF25));
    eprintln!("[$FF26] = {:02x} ($F1) ; NR52", cpu.mmu.read_byte(0xFF26));
    eprintln!("[$FF40] = {:02x} ($91) ; LCDC", cpu.mmu.read_byte(0xFF40));
    eprintln!("[$FF42] = {:02x} ($00) ; SCY ", cpu.mmu.read_byte(0xFF42));
    eprintln!("[$FF43] = {:02x} ($00) ; SCX ", cpu.mmu.read_byte(0xFF43));
    eprintln!("[$FF45] = {:02x} ($00) ; LYC ", cpu.mmu.read_byte(0xFF45));
    eprintln!("[$FF47] = {:02x} ($FC) ; BGP ", cpu.mmu.read_byte(0xFF47));
    eprintln!("[$FF48] = {:02x} ($FF) ; OBP0", cpu.mmu.read_byte(0xFF48));
    eprintln!("[$FF49] = {:02x} ($FF) ; OBP1", cpu.mmu.read_byte(0xFF49));
    eprintln!("[$FF4A] = {:02x} ($00) ; W   ", cpu.mmu.read_byte(0xFF4A));
}

fn init_io_registers(cpu: &mut Cpu) {
    cpu.mmu.write_byte(0x00, 0xFF05);
    cpu.mmu.write_byte(0x00, 0xFF06);
    cpu.mmu.write_byte(0x00, 0xFF07);
    cpu.mmu.write_byte(0x80, 0xFF10);
    cpu.mmu.write_byte(0xBF, 0xFF11);
    cpu.mmu.write_byte(0xF3, 0xFF12);
    cpu.mmu.write_byte(0xBF, 0xFF14);
    cpu.mmu.write_byte(0x3F, 0xFF16);
    cpu.mmu.write_byte(0x00, 0xFF17);
    cpu.mmu.write_byte(0xBF, 0xFF19);
    cpu.mmu.write_byte(0x7F, 0xFF1A);
    cpu.mmu.write_byte(0xFF, 0xFF1B);
    cpu.mmu.write_byte(0x9F, 0xFF1C);
    cpu.mmu.write_byte(0xBF, 0xFF1E);
    cpu.mmu.write_byte(0xFF, 0xFF20);
    cpu.mmu.write_byte(0x00, 0xFF21);
    cpu.mmu.write_byte(0x00, 0xFF22);
    cpu.mmu.write_byte(0xBF, 0xFF23);
    cpu.mmu.write_byte(0x77, 0xFF24);
    cpu.mmu.write_byte(0xF3, 0xFF25);
    cpu.mmu.write_byte(0xF1, 0xFF26);
    cpu.mmu.write_byte(0x91, 0xFF40);
    cpu.mmu.write_byte(0x00, 0xFF42);
    cpu.mmu.write_byte(0x00, 0xFF43);
    cpu.mmu.write_byte(0x00, 0xFF45);
    cpu.mmu.write_byte(0xFC, 0xFF47);
    cpu.mmu.write_byte(0xFF, 0xFF48);
    cpu.mmu.write_byte(0xFF, 0xFF49);
    cpu.mmu.write_byte(0x00, 0xFF4A);
    cpu.mmu.write_byte(0xFF, 0xFF50);   // disable bootrom
}

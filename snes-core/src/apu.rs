mod bootrom;
mod timer;

use crate::apu::timer::{FastTimer, SlowTimer};
use bincode::{Decode, Encode};
use jgenesis_traits::frontend::TimingMode;
use jgenesis_traits::num::GetBit;
use spc700_emu::traits::BusInterface;
use spc700_emu::Spc700;

const AUDIO_RAM_LEN: usize = 64 * 1024;

// Main SNES master clock frequencies
const NTSC_MASTER_CLOCK_FREQUENCY: u64 = 21_477_270;
const PAL_MASTER_CLOCK_FREQUENCY: u64 = 21_281_370;

const APU_MASTER_CLOCK_FREQUENCY: u64 = 24_576_000;

// APU outputs a sample every 24 * 32 master clocks
const SAMPLE_DIVIDER: u8 = 32;

type AudioRam = [u8; AUDIO_RAM_LEN];

#[derive(Debug, Clone, Encode, Decode)]
struct ApuRegisters {
    boot_rom_mapped: bool,
    main_cpu_communication: [u8; 4],
    spc700_communication: [u8; 4],
    timer_0: SlowTimer,
    timer_1: SlowTimer,
    timer_2: FastTimer,
    auxio4: u8,
    auxio5: u8,
}

impl ApuRegisters {
    fn new() -> Self {
        Self {
            boot_rom_mapped: true,
            main_cpu_communication: [0; 4],
            spc700_communication: [0; 4],
            timer_0: SlowTimer::new(),
            timer_1: SlowTimer::new(),
            timer_2: FastTimer::new(),
            auxio4: 0,
            auxio5: 0,
        }
    }

    fn read(&mut self, register: u16) -> u8 {
        log::trace!("SPC700 register read: {register}");

        match register {
            0 => {
                todo!("APU test register read")
            }
            1 => {
                // Control register
                u8::from(self.timer_0.enabled())
                    | (u8::from(self.timer_1.enabled()) << 1)
                    | (u8::from(self.timer_2.enabled()) << 2)
                    | (u8::from(self.boot_rom_mapped) << 7)
            }
            2 | 3 => {
                // TODO DSP registers
                0xFF
            }
            4 => self.main_cpu_communication[0],
            5 => self.main_cpu_communication[1],
            6 => self.main_cpu_communication[2],
            7 => self.main_cpu_communication[3],
            8 => self.auxio4,
            9 => self.auxio5,
            10 => self.timer_0.divider(),
            11 => self.timer_1.divider(),
            12 => self.timer_2.divider(),
            13 => self.timer_0.read_output(),
            14 => self.timer_1.read_output(),
            15 => self.timer_2.read_output(),
            _ => panic!("invalid APU register: {register}"),
        }
    }

    fn write(&mut self, register: u16, value: u8) {
        log::trace!("SPC700 register write: {register} {value:02X}");

        match register {
            0 => {
                todo!("APU test register write")
            }
            1 => {
                // Control register
                self.timer_0.set_enabled(value.bit(0));
                self.timer_1.set_enabled(value.bit(1));
                self.timer_2.set_enabled(value.bit(2));

                if value.bit(4) {
                    self.main_cpu_communication[0] = 0;
                    self.main_cpu_communication[1] = 0;
                }

                if value.bit(5) {
                    self.main_cpu_communication[2] = 0;
                    self.main_cpu_communication[3] = 0;
                }

                self.boot_rom_mapped = value.bit(7);
            }
            2 | 3 => {
                // TODO DSP registers
            }
            4 => {
                self.spc700_communication[0] = value;
            }
            5 => {
                self.spc700_communication[1] = value;
            }
            6 => {
                self.spc700_communication[2] = value;
            }
            7 => {
                self.spc700_communication[3] = value;
            }
            8 => {
                // AUXIO4 register; acts as R/W memory
                self.auxio4 = value;
            }
            9 => {
                // AUXIO5 register; acts as R/W memory
                self.auxio5 = value;
            }
            10 => {
                self.timer_0.set_divider(value);
            }
            11 => {
                self.timer_1.set_divider(value);
            }
            12 => {
                self.timer_2.set_divider(value);
            }
            13..=15 => {
                // Timer outputs; writes do nothing
            }
            _ => panic!("invalid APU register: {register}"),
        }
    }
}

struct Spc700Bus<'a> {
    audio_ram: &'a mut Box<AudioRam>,
    registers: &'a mut ApuRegisters,
}

impl<'a> BusInterface for Spc700Bus<'a> {
    #[inline]
    fn read(&mut self, address: u16) -> u8 {
        match address {
            0x0000..=0x00EF | 0x0100..=0xFFBF => self.audio_ram[address as usize],
            0x00F0..=0x00FF => self.registers.read(address & 0xF),
            0xFFC0..=0xFFFF => {
                if self.registers.boot_rom_mapped {
                    bootrom::SPC700_BOOT_ROM[(address & 0x003F) as usize]
                } else {
                    self.audio_ram[address as usize]
                }
            }
        }
    }

    #[inline]
    fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x00EF | 0x0100..=0xFFFF => {
                self.audio_ram[address as usize] = value;
            }
            0x00F0..=0x00FF => {
                self.registers.write(address & 0xF, value);
                self.audio_ram[address as usize] = value;
            }
        }
    }

    #[inline]
    fn idle(&mut self) {}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ApuTickEffect {
    None,
    OutputSample(f64, f64),
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct Apu {
    spc700: Spc700,
    audio_ram: Box<AudioRam>,
    registers: ApuRegisters,
    main_master_clock_frequency: u64,
    master_cycles_product: u64,
    sample_divider: u8,
}

macro_rules! new_spc700_bus {
    ($self:expr) => {
        Spc700Bus { audio_ram: &mut $self.audio_ram, registers: &mut $self.registers }
    };
}

impl Apu {
    pub fn new(timing_mode: TimingMode) -> Self {
        let main_master_clock_frequency = match timing_mode {
            TimingMode::Ntsc => NTSC_MASTER_CLOCK_FREQUENCY,
            TimingMode::Pal => PAL_MASTER_CLOCK_FREQUENCY,
        };

        let mut apu = Self {
            spc700: Spc700::new(),
            audio_ram: vec![0; AUDIO_RAM_LEN].into_boxed_slice().try_into().unwrap(),
            registers: ApuRegisters::new(),
            main_master_clock_frequency,
            master_cycles_product: 0,
            sample_divider: SAMPLE_DIVIDER,
        };

        apu.spc700.reset(&mut new_spc700_bus!(apu));

        apu
    }

    #[must_use]
    pub fn tick(&mut self, main_master_cycles: u64) -> ApuTickEffect {
        self.master_cycles_product += main_master_cycles * APU_MASTER_CLOCK_FREQUENCY;

        while self.master_cycles_product >= 24 * self.main_master_clock_frequency {
            self.master_cycles_product -= 24 * self.main_master_clock_frequency;
            self.clock();

            self.sample_divider -= 1;
            if self.sample_divider == 0 {
                self.sample_divider = SAMPLE_DIVIDER;
                // TODO output real samples
                return ApuTickEffect::OutputSample(0.0, 0.0);
            }
        }

        ApuTickEffect::None
    }

    fn clock(&mut self) {
        self.spc700.tick(&mut new_spc700_bus!(self));

        self.registers.timer_0.tick();
        self.registers.timer_1.tick();
        self.registers.timer_2.tick();
    }

    pub fn read_port(&mut self, address: u32) -> u8 {
        self.registers.spc700_communication[(address & 0x3) as usize]
    }

    pub fn write_port(&mut self, address: u32, value: u8) {
        self.registers.main_cpu_communication[(address & 0x3) as usize] = value;
    }

    pub fn reset(&mut self) {
        self.registers.boot_rom_mapped = true;
        self.spc700.reset(&mut new_spc700_bus!(self));
    }
}
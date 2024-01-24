//! Game Boy PPU (picture processing unit)

mod fifo;
mod registers;

use crate::interrupts::InterruptRegisters;
use crate::ppu::fifo::PixelFifo;
use crate::ppu::registers::Registers;
use crate::sm83::InterruptType;
use bincode::{Decode, Encode};
use jgenesis_common::frontend::FrameSize;
use jgenesis_common::num::GetBit;
use jgenesis_proc_macros::{FakeDecode, FakeEncode};
use std::ops::{Deref, DerefMut};

const SCREEN_WIDTH: usize = 160;
const SCREEN_HEIGHT: usize = 144;

pub const FRAME_BUFFER_LEN: usize = SCREEN_WIDTH * SCREEN_HEIGHT;

pub const FRAME_SIZE: FrameSize =
    FrameSize { width: SCREEN_WIDTH as u32, height: SCREEN_HEIGHT as u32 };

// 144 rendered lines + 10 VBlank lines
const LINES_PER_FRAME: u8 = 154;
const DOTS_PER_LINE: u16 = 456;
const OAM_SCAN_DOTS: u16 = 80;

const MAX_SPRITES_PER_LINE: usize = 10;

// TODO 16KB for GBC
const VRAM_LEN: usize = 8 * 1024;
const OAM_LEN: usize = 160;

type Vram = [u8; VRAM_LEN];
type Oam = [u8; OAM_LEN];

#[derive(Debug, Clone, FakeEncode, FakeDecode)]
pub struct PpuFrameBuffer(Box<[u8; FRAME_BUFFER_LEN]>);

impl PpuFrameBuffer {
    pub fn iter(&self) -> impl Iterator<Item = u8> + '_ {
        self.0.iter().copied()
    }

    fn set(&mut self, line: u8, pixel: u8, color: u8) {
        self[(line as usize) * SCREEN_WIDTH + (pixel as usize)] = color;
    }
}

impl Default for PpuFrameBuffer {
    fn default() -> Self {
        Self(vec![0; FRAME_BUFFER_LEN].into_boxed_slice().try_into().unwrap())
    }
}

impl Deref for PpuFrameBuffer {
    type Target = [u8; FRAME_BUFFER_LEN];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PpuFrameBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
enum PpuMode {
    // Mode 1
    VBlank,
    // Mode 0
    HBlank,
    // Mode 2
    ScanningOam,
    // Mode 3
    Rendering,
}

impl PpuMode {
    fn to_bits(self) -> u8 {
        match self {
            Self::HBlank => 0,
            Self::VBlank => 1,
            Self::ScanningOam => 2,
            Self::Rendering => 3,
        }
    }
}

#[derive(Debug, Clone, Encode, Decode)]
struct State {
    scanline: u8,
    dot: u16,
    mode: PpuMode,
    stat_interrupt_pending: bool,
    previously_enabled: bool,
    skip_next_frame: bool,
    frame_complete: bool,
}

impl State {
    fn new() -> Self {
        Self {
            scanline: 0,
            dot: 0,
            mode: PpuMode::ScanningOam,
            stat_interrupt_pending: false,
            previously_enabled: true,
            skip_next_frame: true,
            frame_complete: false,
        }
    }

    fn ly(&self) -> u8 {
        if self.scanline == LINES_PER_FRAME - 1 && self.dot >= 4 {
            // LY=0 starts 4 dots into the final scanline. Kirby's Dream Land 2 and Wario Land 2 depend on this for
            // minor top-of-screen effects
            0
        } else {
            self.scanline
        }
    }
}

#[derive(Debug, Clone, Copy, Encode, Decode)]
struct SpriteData {
    oam_index: u8,
    x: u8,
    y: u8,
    tile_number: u8,
    palette: u8,
    horizontal_flip: bool,
    vertical_flip: bool,
    low_priority: bool,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct Ppu {
    frame_buffer: PpuFrameBuffer,
    vram: Box<Vram>,
    oam: Box<Oam>,
    registers: Registers,
    state: State,
    sprite_buffer: Vec<SpriteData>,
    fifo: PixelFifo,
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            frame_buffer: PpuFrameBuffer::default(),
            vram: vec![0; VRAM_LEN].into_boxed_slice().try_into().unwrap(),
            oam: vec![0; OAM_LEN].into_boxed_slice().try_into().unwrap(),
            registers: Registers::new(),
            state: State::new(),
            sprite_buffer: Vec::with_capacity(MAX_SPRITES_PER_LINE),
            fifo: PixelFifo::new(),
        }
    }

    pub fn tick_dot(&mut self, interrupt_registers: &mut InterruptRegisters) {
        if !self.registers.ppu_enabled {
            if self.state.previously_enabled {
                // Disabling the PPU moves it to line 0 + mode 0 and clears the display
                self.state.scanline = 0;
                self.state.dot = 0;
                self.state.mode = PpuMode::HBlank;
                self.frame_buffer.fill(0);

                self.sprite_buffer.clear();
                self.fifo.reset_window_state();
                self.fifo.start_new_line(0, &self.registers, &[]);

                self.state.previously_enabled = false;

                // Signal that the frame should be displayed
                self.state.frame_complete = true;
                return;
            }

            // Unlike TV-based systems, the PPU does not process at all when display is disabled
            return;
        } else if !self.state.previously_enabled {
            self.state.previously_enabled = true;

            // When the PPU is re-enabled, the next frame is not displayed
            self.state.skip_next_frame = true;
        }

        if self.state.stat_interrupt_pending {
            interrupt_registers.set_flag(InterruptType::LcdStatus);
            self.state.stat_interrupt_pending = false;
        }

        let prev_stat_interrupt_line = self.stat_interrupt_line();

        if self.state.mode == PpuMode::Rendering {
            self.fifo.tick(&self.vram, &self.registers, &mut self.frame_buffer);
            if self.fifo.done_with_line() {
                log::trace!(
                    "Pixel FIFO finished line {} after dot {}",
                    self.state.scanline,
                    self.state.dot
                );
                self.state.mode = PpuMode::HBlank;
            }
        }

        self.state.dot += 1;
        if self.state.dot == DOTS_PER_LINE {
            self.state.dot = 0;
            self.state.scanline += 1;
            if self.state.scanline == LINES_PER_FRAME {
                self.state.scanline = 0;
                self.fifo.reset_window_state();
            }

            if self.state.scanline < SCREEN_HEIGHT as u8 {
                self.state.mode = PpuMode::ScanningOam;

                scan_oam(
                    self.state.scanline,
                    self.registers.double_height_sprites,
                    &self.oam,
                    &mut self.sprite_buffer,
                );

                // Reset the FIFO here so that the WY check happens at the start of mode 2 rather than start of mode 3
                self.fifo.start_new_line(self.state.scanline, &self.registers, &self.sprite_buffer);
            } else {
                self.state.mode = PpuMode::VBlank;
            }
        } else if self.state.scanline < SCREEN_HEIGHT as u8 && self.state.dot == OAM_SCAN_DOTS {
            self.state.mode = PpuMode::Rendering;
        }

        // TODO timing
        if self.state.scanline == SCREEN_HEIGHT as u8 && self.state.dot == 1 {
            interrupt_registers.set_flag(InterruptType::VBlank);
            if self.state.skip_next_frame {
                self.state.skip_next_frame = false;
            } else {
                self.state.frame_complete = true;
            }
        }

        let stat_interrupt_line = self.stat_interrupt_line();
        if !prev_stat_interrupt_line && stat_interrupt_line {
            self.state.stat_interrupt_pending = true;
        }
    }

    fn stat_interrupt_line(&self) -> bool {
        (self.registers.lyc_interrupt_enabled && self.state.ly() == self.registers.ly_compare)
            || (self.registers.mode_2_interrupt_enabled && self.state.mode == PpuMode::ScanningOam)
            || (self.registers.mode_1_interrupt_enabled && self.state.mode == PpuMode::VBlank)
            || (self.registers.mode_0_interrupt_enabled && self.state.mode == PpuMode::HBlank)
    }

    pub fn frame_buffer(&self) -> &PpuFrameBuffer {
        &self.frame_buffer
    }

    pub fn frame_complete(&self) -> bool {
        self.state.frame_complete
    }

    pub fn clear_frame_complete(&mut self) {
        self.state.frame_complete = false;
    }

    pub fn read_vram(&self, address: u16) -> u8 {
        // TODO banking for GBC
        // TODO VRAM blocking
        self.vram[(address & 0x1FFF) as usize]
    }

    pub fn write_vram(&mut self, address: u16, value: u8) {
        // TODO banking for GBC
        // TODO VRAM blocking
        self.vram[(address & 0x1FFF) as usize] = value;
    }

    pub fn read_oam(&self, address: u16) -> u8 {
        // TODO OAM blocking
        self.oam[(address & 0xFF) as usize]
    }

    pub fn write_oam(&mut self, address: u16, value: u8) {
        // TODO OAM blocking
        self.oam[(address & 0xFF) as usize] = value;
    }

    pub fn read_register(&self, address: u16) -> u8 {
        match address & 0xFF {
            0x40 => self.registers.read_lcdc(),
            0x41 => self.registers.read_stat(self.state.scanline, self.state.mode),
            0x42 => self.registers.bg_y_scroll,
            0x43 => self.registers.bg_x_scroll,
            // LY: Line number
            0x44 => self.state.ly(),
            0x45 => self.registers.ly_compare,
            0x47 => self.registers.read_bgp(),
            0x48 => self.registers.read_obp0(),
            0x49 => self.registers.read_obp1(),
            0x4A => self.registers.window_y,
            0x4B => self.registers.window_x,
            _ => {
                log::warn!("PPU register read {address:04X}");
                0xFF
            }
        }
    }

    pub fn write_register(&mut self, address: u16, value: u8) {
        log::trace!(
            "PPU register write on line {} dot {}: {address:04X} set to {value:02X}",
            self.state.scanline,
            self.state.dot
        );

        match address & 0xFF {
            0x40 => self.registers.write_lcdc(value),
            0x41 => self.registers.write_stat(value),
            0x42 => self.registers.write_scy(value),
            0x43 => self.registers.write_scx(value),
            // LY, not writable
            0x44 => {}
            0x45 => self.registers.write_lyc(value),
            0x47 => self.registers.write_bgp(value),
            0x48 => self.registers.write_obp0(value),
            0x49 => self.registers.write_obp1(value),
            0x4A => self.registers.write_wy(value),
            0x4B => self.registers.write_wx(value),
            _ => log::warn!("PPU register write {address:04X} {value:02X}"),
        }
    }
}

fn scan_oam(
    scanline: u8,
    double_height_sprites: bool,
    oam: &Oam,
    sprite_buffer: &mut Vec<SpriteData>,
) {
    sprite_buffer.clear();

    let sprite_height = if double_height_sprites { 16 } else { 8 };

    for oam_idx in 0..OAM_LEN / 4 {
        let oam_addr = 4 * oam_idx;

        let y = oam[oam_addr];

        // Check if sprite overlaps current line
        let sprite_top = i16::from(y) - 16;
        let sprite_bottom = sprite_top + sprite_height;
        if !(sprite_top..sprite_bottom).contains(&scanline.into()) {
            continue;
        }

        let x = oam[oam_addr + 1];
        let tile_number = oam[oam_addr + 2];

        let attributes = oam[oam_addr + 3];
        let palette: u8 = attributes.bit(4).into();
        let horizontal_flip = attributes.bit(5);
        let vertical_flip = attributes.bit(6);
        let low_priority = attributes.bit(7);

        sprite_buffer.push(SpriteData {
            oam_index: oam_idx as u8,
            x,
            y,
            tile_number,
            palette,
            horizontal_flip,
            vertical_flip,
            low_priority,
        });
        if sprite_buffer.len() == MAX_SPRITES_PER_LINE {
            break;
        }
    }

    sprite_buffer.sort_by(|a, b| a.x.cmp(&b.x).then(a.oam_index.cmp(&b.oam_index)));
}
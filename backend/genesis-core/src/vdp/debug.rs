use crate::vdp;
use crate::vdp::{
    gen_color_to_rgb, read_pattern_generator, resolve_color, ColorModifier, PatternGeneratorArgs,
    Vdp,
};

use jgenesis_common::frontend::Color;

impl Vdp {
    pub fn copy_cram(&self, out: &mut [Color]) {
        for (color, chunk) in out.iter_mut().zip(self.cram.chunks_exact(2)) {
            let &[msb, lsb] = chunk else { unreachable!("chunks_exact(2)") };
            let gen_color = u16::from_be_bytes([msb, lsb]);
            *color = parse_gen_color(gen_color);
        }
    }

    pub fn copy_vram(&self, out: &mut [Color], palette: u8, row_len: usize) {
        for pattern in 0..vdp::VRAM_LEN / 32 {
            let base_idx = pattern / row_len * row_len * 64 + (pattern % row_len) * 8;

            for row in 0..8 {
                for col in 0..8 {
                    let out_idx = base_idx + row * row_len * 8 + col;

                    let color_id = read_pattern_generator(
                        &self.vram,
                        PatternGeneratorArgs {
                            vertical_flip: false,
                            horizontal_flip: false,
                            pattern_generator: pattern as u16,
                            row: row as u16,
                            col: col as u16,
                            cell_height: 8,
                        },
                    );
                    let color = resolve_color(&self.cram, palette, color_id);
                    out[out_idx] = parse_gen_color(color);
                }
            }
        }
    }
}

fn parse_gen_color(gen_color: u16) -> Color {
    let r = ((gen_color >> 1) & 0x07) as u8;
    let g = ((gen_color >> 5) & 0x07) as u8;
    let b = ((gen_color >> 9) & 0x07) as u8;
    gen_color_to_rgb(r, g, b, ColorModifier::None, false)
}
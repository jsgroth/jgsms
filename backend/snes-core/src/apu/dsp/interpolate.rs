use crate::apu::dsp::tables;

const I15_MIN: f64 = -16384.0;
const I15_MAX: f64 = 16383.0;

pub struct InterpolateArgs {
    pub pitch_counter: u16,
    pub oldest: i16,
    pub older: i16,
    pub old: i16,
    pub sample: i16,
}

pub fn gaussian(
    InterpolateArgs { pitch_counter, oldest, older, old, sample }: InterpolateArgs,
) -> i16 {
    // Do math in 32 bits to avoid overflows
    let sample: i32 = sample.into();
    let old: i32 = old.into();
    let older: i32 = older.into();
    let oldest: i32 = oldest.into();

    // Bits 4-11 of pitch counter are the interpolation index into the Gaussian interpolation table
    let interpolation_idx = ((pitch_counter >> 4) & 0xFF) as usize;

    // Sum the 3 older samples with 15-bit wrapping
    let mut sum = (tables::GAUSSIAN[0x0FF - interpolation_idx] * oldest) >> 11;
    sum += (tables::GAUSSIAN[0x1FF - interpolation_idx] * older) >> 11;
    sum += (tables::GAUSSIAN[0x100 + interpolation_idx] * old) >> 11;

    // Clip to 15 bits
    sum = (((sum as i16) << 1) >> 1).into();

    // Add in the current sample
    sum += (tables::GAUSSIAN[interpolation_idx] * sample) >> 11;

    // Clamp the final result to signed 15-bit
    sum.clamp((i16::MIN >> 1).into(), (i16::MAX >> 1).into()) as i16
}

// Based on https://yehar.com/blog/wp-content/uploads/2009/08/deip.pdf
pub fn hermite(
    InterpolateArgs { pitch_counter, oldest, older, old, sample }: InterpolateArgs,
) -> i16 {
    let y3: f64 = sample.into();
    let y2: f64 = old.into();
    let y1: f64 = older.into();
    let y0: f64 = oldest.into();
    let x = f64::from(pitch_counter & 0xFFF) / 4096.0;

    let c0 = y1;
    let c1 = 0.5 * (y2 - y0);
    let c2 = y0 - 2.5 * y1 + 2.0 * y2 - 0.5 * y3;
    let c3 = 0.5 * (y3 - y0) + 1.5 * (y1 - y2);
    (((c3 * x + c2) * x + c1) * x + c0).round().clamp(I15_MIN, I15_MAX) as i16
}

#![no_std]

use bitvec::field::BitField;
use bitvec::order::Lsb0;
use bitvec::view::BitView;

pub fn to_sb_channels(buf: &[u8]) -> [f32; 16] {
    let mut each = [0.; 16];
    let view = buf[1..=22].view_bits::<Lsb0>();
    for t in (0..11 * 16).step_by(11) {
        each[t / 11] = view[t..t + 11].load_le::<u16>() as f32 - 192.
    }
    each
}

pub fn norm_perc(buf: &[u8], ch: usize) -> f32 {
    (to_sb_channels(&buf)[ch] / 16.).clamp(0., 100.)
}

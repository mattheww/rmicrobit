//! A 5×5 ascii font.
//!
//! This is a copy of the 'pendolino' font from the [micro:bit runtime][dal].
//!
//! [dal]: https://lancaster-university.github.io/microbit-docs/

mod pendolino;

use crate::graphics::image::BitImage;

/// Index of the first character in the standard font
pub const PRINTABLE_START: usize = 32;

/// Number of characters in the standard font
pub const PRINTABLE_COUNT: usize = 95;

const UNKNOWN: BitImage = BitImage::new(&[
    [1, 1, 1, 1, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 1, 1, 1, 1],
]);


/// Returns an image representing the requested ascii character.
///
/// If the requested character isn't printable, returns a 'hollow square' image.
///
/// # Example
///
/// `font::character(b'x')`
pub fn character(index: u8) -> &'static BitImage {
    let index = index as usize;
    if index < PRINTABLE_START || index >= PRINTABLE_START + PRINTABLE_COUNT {
        return &UNKNOWN;
    }
    &self::pendolino::PENDOLINO3[index - PRINTABLE_START]
}

const fn font_entry(data: [u8; 5]) -> BitImage {
    // Note the data in the pendolino font uses the opposite column numbering
    // system to BitImage.
    const fn row_bits(byte: u8) -> [u8; 5] {[
        ((byte & 1<<4) != 0) as u8,
        ((byte & 1<<3) != 0) as u8,
        ((byte & 1<<2) != 0) as u8,
        ((byte & 1<<1) != 0) as u8,
        ((byte & 1<<0) != 0) as u8,
    ]}
    BitImage::new(&[
        row_bits(data[0]),
        row_bits(data[1]),
        row_bits(data[2]),
        row_bits(data[3]),
        row_bits(data[4]),
    ])
}


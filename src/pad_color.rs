use clap::ValueEnum;

#[derive(ValueEnum, Clone, Copy)]
pub enum PadColor {
    Black,
    White,
}

impl From<PadColor> for image::Rgb<u8> {
    fn from(value: PadColor) -> image::Rgb<u8> {
        match value {
            PadColor::Black => image::Rgb([0, 0, 0]),
            PadColor::White => image::Rgb([255, 255, 255]),
        }
    }
}

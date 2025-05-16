use crate::{Area, Point};

#[derive(Debug)]
pub enum PixelabError {
    PointOutOfBounds {
        point: Point,
        bound: Area,
    },
    BitmapOutOfBounds {
        point: Point,
        main_bound: Area,
        sub_bound: Area,
    },
    BitmapFormatMismatch,
    InsufficientPoints,
    BitmapBufferNotCreated,
    BitmapTypeIsNone,
    EmptyEvent,
}

impl core::error::Error for PixelabError {}
impl core::fmt::Display for PixelabError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::PointOutOfBounds { point, bound } => {
                write!(f, "PointOutOfBounds Point: {}, Bound: {}", point, bound)
            }
            Self::BitmapOutOfBounds {
                point,
                main_bound,
                sub_bound,
            } => {
                write!(
                    f,
                    "BitmapOutOfBounds Point: {}, Main-Bound: {}, Sub-Bound: {}",
                    point, main_bound, sub_bound
                )
            }
            Self::BitmapFormatMismatch => {
                write!(f, "FormatMismatch")
            }
            Self::InsufficientPoints => {
                write!(f, "InsufficientPoints")
            }
            Self::BitmapBufferNotCreated => {
                write!(f, "BitMapBufferNotCreated")
            }
            Self::BitmapTypeIsNone => {
                write!(f, "BitMapTypeIsNone")
            }
            Self::EmptyEvent => {
                write!(f, "EmptyEvent")
            }
        }
    }
}

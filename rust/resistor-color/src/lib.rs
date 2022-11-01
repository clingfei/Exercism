extern crate int_enum;
extern crate enum_iterator;

use int_enum::IntEnum;
use enum_iterator::{all, Sequence};

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, Sequence, Clone, Copy, IntEnum)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

static STRING:[&str; 10] = ["Black", "Brown", "Red", "Orange",  "Yellow", "Green", "Blue", "Violet", "Grey", "White", ];

pub fn color_to_value(_color: ResistorColor) -> u32 {
    _color.int_value()
}

pub fn value_to_color_string(value: u32) -> String {
    if ResistorColor::from_int(value).is_err() {
        return "value out of range".to_string();
    }
    STRING[value as usize].to_string()
}

pub fn colors() -> Vec<ResistorColor> {
    all::<ResistorColor>().collect::<Vec<_>>()
}

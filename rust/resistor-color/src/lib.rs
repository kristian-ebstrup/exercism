use enum_iterator::{all, Sequence};
use int_enum::IntEnum;

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, IntEnum, Sequence)]
pub enum ResistorColor {
    Black = 0,
    Blue = 6,
    Brown = 1,
    Green = 5,
    Grey = 8,
    Orange = 3,
    Red = 2,
    Violet = 7,
    White = 9,
    Yellow = 4,
}

// implement Display trait
impl std::fmt::Display for ResistorColor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ResistorColor::Black => write!(f, "Black"),
            ResistorColor::Blue => write!(f, "Blue"),
            ResistorColor::Brown => write!(f, "Brown"),
            ResistorColor::Green => write!(f, "Green"),
            ResistorColor::Grey => write!(f, "Grey"),
            ResistorColor::Orange => write!(f, "Orange"),
            ResistorColor::Red => write!(f, "Red"),
            ResistorColor::Violet => write!(f, "Violet"),
            ResistorColor::White => write!(f, "White"),
            ResistorColor::Yellow => write!(f, "Yellow"),
        }
    }
}

pub fn color_to_value(_color: ResistorColor) -> u32 {
    // create iterator for enum
    let iter = all::<ResistorColor>().collect::<Vec<_>>();

    // compare input _color with enum, and return value
    for color in iter {
        if _color == color {
            return color.int_value();
        }
    };

    // if no color found, throw a fit
    panic!("unidentified color input");
}

pub fn value_to_color_string(value: u32) -> String {
    // input is a u32 int
    // ouput must be a string
    // but the intermediary is the enum ResistorColor
    
    // check if input value is out of range
    if value > 9 { return String::from("value out of range") };

    // create iterator for enum
    let iter = all::<ResistorColor>().collect::<Vec<_>>();

    // compare input value with enum.int_value(), and return string
    for color in iter {
        if value == color.int_value() {
            return color.to_string();
        }
    };

    panic!("unidentified value input");
}

pub fn colors() -> Vec<ResistorColor> {
    // create iterator for enum
    let iter = all::<ResistorColor>().collect::<Vec<_>>();

    // push all values unto an empty vector
    let mut values: Vec<u32> = Vec::new();
    for color in &iter {
        values.push(color.int_value());
    }

    // sort the new vector, and reverse it to allow using pop()
    values.sort();
    values.reverse();

    // loop through iterator until values is empty
    // and allocate <ResistorColor> in a new vec
    let mut sorted_colors: Vec<ResistorColor> = Vec::new();
    while let Some(top) = values.pop() {
        for color in &iter {
            if color.int_value() == top {
                sorted_colors.push(*color);
            }
        }
    }

    return sorted_colors;
}

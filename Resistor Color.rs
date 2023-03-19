#[derive(Debug, PartialEq, Eq)]
pub enum ResistorColor {
    Black,
    Blue,
    Brown,
    Green,
    Grey,
    Orange,
    Red,
    Violet,
    White,
    Yellow,
}

fn string_color_to_enum_color(color: &str) -> ResistorColor {
    match color {
        "Black" => ResistorColor::Black,
        "Blue" => ResistorColor::Blue,
        "Brown" => ResistorColor::Brown,
        "Green" => ResistorColor::Green,
        "Grey" => ResistorColor::Grey,
        "Orange" => ResistorColor::Orange,
        "Red" => ResistorColor::Red,
        "Violet" => ResistorColor::Violet,
        "White" => ResistorColor::White,
        "Yellow" => ResistorColor::Yellow,
        _ => panic!("Invalid value for resistor color."),
    }
}

pub fn color_to_value(_color: ResistorColor) -> u32 {
    let mut res = 0;
    match _color {
        ResistorColor::Black => res = 0,
        ResistorColor::Brown => res = 1,
        ResistorColor::Red => res = 2,
        ResistorColor::Orange => res = 3,
        ResistorColor::Yellow => res = 4,
        ResistorColor::Green => res = 5,
        ResistorColor::Blue => res = 6,
        ResistorColor::Violet => res = 7,
        ResistorColor::Grey => res = 8,
        ResistorColor::White => res = 9,
    };
    return res;
}

pub fn value_to_color_string(value: u32) -> String {
    let mut res = String::new();
    match value {
        0 => res = String::from("Black"),
        1 => res = String::from("Brown"),
        2 => res = String::from("Red"),
        3 => res = String::from("Orange"),
        4 => res = String::from("Yellow"),
        5 => res = String::from("Green"),
        6 => res = String::from("Blue"),
        7 => res = String::from("Violet"),
        8 => res = String::from("Grey"),
        9 => res = String::from("White"),
        _ => res = String::from("value out of range"),
    };
    return res;
}

pub fn colors() -> Vec<ResistorColor> {
    let mut res: Vec<ResistorColor> = Vec::new();
    for number in 0..=9 {
        res.push(string_color_to_enum_color(value_to_color_string(number).as_str()));
    }
    return res;
}

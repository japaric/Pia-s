#[derive(Clone, Copy)]
pub enum Class {
    ColumnContainer,
    Console,
    Degree,
    HvCenter,
    Octave,
    OutOfKey,
    Overtone,
    Padded,
    Piano,
    PianoBlack,
    PianoColor,
    PianoWhite,
    Pressed,
    RowContainer,
    Sustained,
}

// TODO compress `style.css` to not use these long names
impl Class {
    pub fn as_str(&self) -> &'static str {
        use Class::*;

        match self {
            ColumnContainer => "column-container",
            Console => "console",
            Degree => "degree",
            HvCenter => "hv-center",
            Octave => "octave",
            Overtone => "overtone",
            OutOfKey => "out-of-key",
            Padded => "padded",
            Piano => "piano",
            PianoBlack => "piano-black",
            PianoColor => "piano-color",
            PianoWhite => "piano-white",
            Pressed => "pressed",
            RowContainer => "row-container",
            Sustained => "sustained",
        }
    }
}

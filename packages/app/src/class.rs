#[derive(Clone, Copy)]
pub enum Class {
    CofLabel,
    CofPath,
    ColumnContainer,
    Console,
    Contour,
    ContourGridMajor,
    ContourGridMinor,
    ContourLine,
    Degree,
    Highlight,
    HvCenter,
    InScale,
    LilWarning,
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
    Tonnetz,
    TonnetzCircle,
    TonnetzLabel,
}

// TODO compress `style.css` to not use these long names
impl Class {
    pub fn as_str(&self) -> &'static str {
        use Class::*;

        match self {
            CofPath => "cof-path",
            CofLabel => "cof-label",
            ColumnContainer => "column-container",
            Contour => "contour",
            ContourGridMajor => "contour-grid-major",
            ContourGridMinor => "contour-grid-minor",
            ContourLine => "contour-line",
            Console => "console",
            Degree => "degree",
            Highlight => "highlight",
            HvCenter => "hv-center",
            InScale => "in-scale",
            LilWarning => "lil-warning",
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
            Tonnetz => "tonnetz",
            TonnetzCircle => "tonnetz-circle",
            TonnetzLabel => "tonnetz-label",
        }
    }
}

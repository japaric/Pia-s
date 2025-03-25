#[derive(Clone, Copy)]
#[cfg_attr(test, derive(Debug, PartialEq))]
#[allow(non_camel_case_types)]
pub enum Interval {
    P0,
    m2,
    M2,
    m3,
    M3,
    P4,
    TT,
    P5,
    m6,
    M6,
    m7,
    M7,
}

impl Interval {
    pub fn from_i8_lossy(mut value: i8) -> Self {
        use Interval::*;

        value %= 12;
        if value < 0 {
            value += 12;
        }

        match value {
            0 => P0,
            1 => m2,
            2 => M2,
            3 => m3,
            4 => M3,
            5 => P4,
            6 => TT,
            7 => P5,
            8 => m6,
            9 => M6,
            10 => m7,
            _ => M7,
        }
    }

    pub fn as_str(&self) -> &'static str {
        use Interval::*;

        match self {
            P0 => "P0",
            m2 => "m2",
            M2 => "M2",
            m3 => "m3",
            M3 => "M3",
            P4 => "P4",
            TT => "TT",
            P5 => "P5",
            m6 => "m6",
            M6 => "M6",
            m7 => "m7",
            M7 => "M7",
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn from_i8_lossy() {
        use Interval::*;

        assert_eq!(P0, Interval::from_i8_lossy(0));

        assert_eq!(m2, Interval::from_i8_lossy(1));
        assert_eq!(m2, Interval::from_i8_lossy(13));

        assert_eq!(M7, Interval::from_i8_lossy(11));
        assert_eq!(M7, Interval::from_i8_lossy(23));

        assert_eq!(M7, Interval::from_i8_lossy(-1));
        assert_eq!(M7, Interval::from_i8_lossy(-13));

        assert_eq!(m2, Interval::from_i8_lossy(-11));
        assert_eq!(m2, Interval::from_i8_lossy(-23));
    }
}

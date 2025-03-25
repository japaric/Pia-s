use music::NoteName;

#[derive(Clone, Copy)]
pub struct ScaleFactor {
    num_white_keys: usize,
}

impl ScaleFactor {
    pub fn new(num_white_keys: usize) -> Self {
        Self { num_white_keys }
    }

    pub fn white_key_width(&self) -> f64 {
        1. / self.num_white_keys as f64
    }

    pub fn black_key_width(&self) -> f64 {
        self.white_key_width() * 14. / 24.
    }

    pub fn width(&self, note: NoteName) -> f64 {
        use NoteName::*;

        match note {
            C | E => self.black_key_padding3(),
            F | B => self.black_key_padding5(),
            _ => self.black_key_width(),
        }
    }

    pub fn offset(&self, note: NoteName) -> f64 {
        use NoteName::*;

        match note {
            A => {
                2. * self.white_key_width()
                    - self.black_key_padding5()
                    - 2. * self.black_key_width()
            }

            C => 0.,

            _ => unimplemented!(),
        }
    }

    fn black_key_padding3(&self) -> f64 {
        (3. * self.white_key_width() - 3. * self.black_key_width()) / 2.
    }

    fn black_key_padding5(&self) -> f64 {
        (4. * self.white_key_width() - 5. * self.black_key_width()) / 2.
    }
}

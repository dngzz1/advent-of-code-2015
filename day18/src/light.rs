use std::fmt;

#[derive(Debug)]
pub enum Light {
    On,
    Off,
}

impl Light {
    pub fn symbol(&self) -> char {
        return match *self {
            Light::On => '#',
            Light::Off => '.',
        };
    }
}

impl fmt::Display for Light {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

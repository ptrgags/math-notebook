pub struct UnitBlade {
    blades: u8,
}

impl UnitBlade {
    pub fn new(blades: u8) -> Self {
        Self {
            blades
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
}
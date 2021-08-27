use crate::vector::*;

/// A pixel unit. This is the main protagonist of our game :)
pub struct Pixel {
    /// The screen position vector of the pixel
    pub position: Vector,

    /// Current velocity of this pixel
    pub velocity: Vector,
}

impl Pixel {
    /// Spawns a new pixel
    pub fn new(position: Vector) -> Self {
        Self {
            velocity: Vector::from(0.),
            position,
        }
    }
}

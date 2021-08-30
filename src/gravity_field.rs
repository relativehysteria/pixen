use crate::vector::*;

/// A gravity field in the arena.
/// The field can either attract or repel.
pub struct GravityField {
    /// Position of the gravity field
    pub position: Vector,

    /// Area of effect of this field. Pixels farther than `aoe` won't be
    /// affected by it.
    pub aoe: f32,

    /// Gravitational strength applied to pixels affected by this field.
    /// The higher the strength, the faster the affected pixels accelerate.
    pub strength: f32,
}

impl GravityField {
    /// Spawns a new gravity field
    pub fn new(position: Vector, aoe: f32, strength: f32) -> Self {
        Self {
            position,
            aoe,
            strength,
        }
    }

    /// Checks whether a vector is inside the `aoe` of this field
    pub fn in_aoe(&self, vector: &Vector) -> bool {
        self.position.distance(vector) <= self.aoe
    }
}

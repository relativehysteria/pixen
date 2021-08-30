#[derive(Copy, Clone)]
/// A generic vector
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Vector {
    /// Creates a new vector
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y, }
    }

    /// Creates a new vector from a single float
    pub fn from(num: f32) -> Self {
        Self { x: num, y: num, }
    }


    /// Creates a new vector
    pub fn coords(coords: (f32, f32)) -> Self {
        let (x, y) = coords;
        Self { x, y, }
    }

    /// Clamps the `x` and `y` coordinates within 0 +/- `limit`
    pub fn limit(&mut self, limit: f32) {
        self.x = self.x.clamp(0. - limit, 0. + limit);
        self.y = self.y.clamp(0. - limit, 0. + limit);
    }

    /// Returns the magnitude (length) of a vector
    pub fn magnitude(&self) -> f32 {
        return f32::sqrt(self.x * self.x + self.y * self.y);
    }

    /// Normalizes the vector. That is, changes its length to 1 but keeps the
    /// direction the same.
    pub fn normalize(&mut self) {
        let mag = self.magnitude();
        if mag != 0. {
            *self /= Vector::new(mag, mag);
        }
    }

    /// Returns the distance between two vectors.
    pub fn distance(&self, other: &Vector) -> f32 {
        f32::sqrt((other.x - self.x).powf(2.) + (other.y - self.y).powf(2.))
    }

    /// Sets the vector to (0,0).
    pub fn clear(&mut self) {
        self.x = 0.;
        self.y = 0.;
    }
}

impl core::ops::AddAssign for Vector {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl core::ops::Add for Vector {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl core::ops::SubAssign for Vector {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl core::ops::Sub for Vector {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl core::ops::MulAssign for Vector {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl core::ops::Mul for Vector {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl core::ops::DivAssign for Vector {
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl core::ops::Div for Vector {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

use jotter::Vector;

#[derive(Debug, Default, Clone, Copy)]
pub struct Complex {
    pub real: f32,
    pub imaginary: f32,
}

impl Complex {
    pub fn new(real: f32, imaginary: f32) -> Self {
        Self { real, imaginary }
    }

    pub fn abs(&self) -> f32 {
        (self.real * self.real + self.imaginary * self.imaginary).sqrt()
    }
}

impl std::ops::Add<Complex> for Complex {
    type Output = Self;

    fn add(self, rhs: Complex) -> Self {
        Self {
            real: self.real + rhs.real,
            imaginary: self.imaginary + rhs.imaginary,
        }
    }
}

impl std::ops::Mul<Complex> for Complex {
    type Output = Self;

    fn mul(self, rhs: Complex) -> Self {
        Self {
            real: self.real * rhs.real - self.imaginary * rhs.imaginary,
            imaginary: self.real * rhs.imaginary + self.imaginary * rhs.real,
        }
    }
}

impl From<Complex> for Vector {
    fn from(complex: Complex) -> Self {
        Self::new(complex.real, complex.imaginary)
    }
}

impl From<Vector> for Complex {
    fn from(vector: Vector) -> Self {
        Self::new(vector.x, vector.y)
    }
}

impl From<f32> for Complex {
    fn from(n: f32) -> Self {
        Self::new(n, 0.0)
    }
}
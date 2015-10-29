use std::ops;


struct M<T: Clone + ops::Add + ops::Mul> {
    dim: (usize, usize),
    data: Vec<Vec<T>>,
}

impl <T: Clone + ops::Add + ops::Mul> M<T>  {
    fn new(x: usize, y: usize, z: T) -> Self {
        M {
            dim: (x, y),
            data: vec![vec![z; y]; x],
        }
    }
}

impl <T: Clone + ops::Add + ops::Mul> ops::Add for M<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {

    }
}

impl <T: Clone + ops::Add + ops::Mul> ops::Mul for M<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {

    }
}


struct V<T>(Vec<T>);

impl<T: Clone + ops::Add + ops::Mul> ops::Mul for V<T> {
    type Output = T;

    fn mul(self, rhs: Self) -> T {
        self.0.iter().zip(rhs.0.iter()).map(|(x, y)| x*y).sum();
    }
}

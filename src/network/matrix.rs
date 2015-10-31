use std::ops;


///# Matrix
struct M<T> {
    dim: (usize, usize),
    data: V<V<T>>,
}

impl <T: Clone> M<T>  {
    fn new(x: usize, y: usize, z: V<V<T>>) -> Self {
        M {
            dim: (x, y),
            data: z,
        }
    }

    fn defaultNew(x: usize, y: usize, z: T) -> Self {
        M::new(x, y, V::new(vec![V::new(vec![z; y]); x]))
    }
}

// impl <T: ops::Add> ops::Add for M<T> {
//     type Output = Self;
//
//     fn add(self, rhs: Self) -> Self {
//
//     }
// }
//
// impl <T: ops::Sub> ops::Sub for M<T> {
//     type Output = Self;
//
//     fn sub(self, rhs: Self) -> Self {
//
//     }
// }
//
// impl <T: ops::Mul + ops::Add> ops::Mul for M<T> {
//     type Output = Self;
//
//     fn mul(self, rhs: Self) -> Self {
//
//     }
// }
//
// impl <T: ops::Neg> ops::Neg for M<T> {
//     type Output = Self;
//
//     fn neg(self) -> Self {
//
//     }
// }


///# Vector
#[derive(Clone)]
struct V<T> {
    data: Vec<T>,
}

impl <T> V<T> {
    fn new(z: Vec<T>) -> Self {
        V {
            data: z,
        }
    }

    fn len(self) -> usize {
        self.data.len()
    }
}

// impl<T: ops::Add> ops::Add for V<T> {
//     type Output = T;
//
//     fn add(self, rhs: Self) -> T {
//
//     }
// }
//
// impl<T: ops::Sub> ops::Sub for V<T> {
//     type Output = T;
//
//     fn sub(self, rhs: Self) -> T {
//
//     }
// }

impl<T: ops::Mul<Output=T> + ops::Add<Output=T> + Clone> ops::Mul for V<T> {
    type Output = T;

    fn mul(self, rhs: Self) -> T {
        let data_iter = self.data.iter().zip(rhs.data.iter()).skip(1);
        let first = self.data[0].clone() * rhs.data[0].clone();
        data_iter.fold(first, |sum, (x, y)| x.clone()*y.clone()+sum)
    }
}

// impl <T: ops::Neg> ops::Neg for V<T> {
//     type Output = Self;
//
//     fn neg(self) -> Self {
//
//     }
// }

use std::ops;


///# Matrix
#[derive(Clone)]
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

    fn default_new(x: usize, y: usize, z: T) -> Self {
        M::new(x, y, V::new(vec![V::new(vec![z; y]); x]))
    }

    fn transpose(self) -> M<T> {
        let mut rows: Vec<V<T>> = Vec::with_capacity(self.dim.1);
        for i in 0..self.dim.1 {
            let mut cols: Vec<T> = Vec::with_capacity(self.dim.0);
            for j in 0..self.dim.0 {
                cols.push(self.data.0[j].0[i].clone());
            }
            rows.push(V(cols));
        }
        M::new(self.dim.1, self.dim.0, V(rows))
    }
}

impl <T: ops::Mul<Output=T> + Clone> M<T> {
    // fn inner(self, rhs: Self) -> M<T> {
    //
    // }
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
struct V<T>(Vec<T>);

impl <T> V<T> {
    fn new(z: Vec<T>) -> Self {
        V(z)
    }

    fn len(self) -> usize {
        self.0.len()
    }
}

impl <T: ops::Mul<Output=T> + Clone> V<T> {
    fn inner(self, rhs: Self) -> V<T> {
        V(self.0.iter().zip(rhs.0.iter()).map(|(x, y)| x.clone() * y.clone()).collect())
    }
}

impl<T: ops::Add<Output=T> + Clone> ops::Add for V<T> {
    type Output = V<T>;

    fn add(self, rhs: Self) -> V<T> {
        V(self.0.iter().zip(rhs.0.iter()).map(|(x, y)| x.clone() + y.clone()).collect())
    }
}

impl<T: ops::Sub<Output=T> + Clone> ops::Sub for V<T> {
    type Output = V<T>;

    fn sub(self, rhs: Self) -> V<T> {
        V(self.0.iter().zip(rhs.0.iter()).map(|(x, y)| x.clone() - y.clone()).collect())
    }
}

impl<T: ops::Mul<Output=T> + ops::Add<Output=T> + Clone> ops::Mul for V<T> {
    type Output = T;

    fn mul(self, rhs: Self) -> T {
        let data_iter = self.0.iter().zip(rhs.0.iter()).skip(1);
        let first = self.0[0].clone() * rhs.0[0].clone();
        data_iter.fold(first, |sum, (x, y)| x.clone()*y.clone()+sum)
    }
}

impl <T: ops::Neg<Output=T> + Clone> ops::Neg for V<T> {
    type Output = Self;

    fn neg(self) -> Self {
        V(self.0.iter().map(|x| -x.clone()).collect())
    }
}

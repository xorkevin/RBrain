struct M<T> {
    dim: (usize, usize),
    data: [[T]],
}

impl <T> M<T>  {
    fn new(x: usize, y: usize, z: T) -> M<T> {
        M {
            dim: (x, y),
            data: [[z; y]; x]
        }
    }
}

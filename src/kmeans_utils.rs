use simd_euclidean::Vectorized;

// Generic Vectorized is implicit a reference. Not confusing at all.

pub fn euclidian_distance<T: Vectorized>(a: T, b: T) -> <T as Vectorized>::Output {
    return Vectorized::distance(a,b);
}

pub fn squared_distance<T: Vectorized>(a: T, b: T) -> <T as Vectorized>::Output {
    return Vectorized::squared_distance(a, b);
}

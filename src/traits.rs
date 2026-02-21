/// Compute the dynamic time warping of two sequence.
pub trait Solution<D> {
    /// Warped distance between `a` and `b`.
    fn distance(&self) -> D;

    /// Warped path between `a` and `b`.
    fn path(&self) -> Vec<(usize, usize)>;
}

/// An arbitrary distance between two objects.
pub trait Distance {
    type Output;
    /// Distance between `self` and `other`.
    fn distance(&self, other: &Self) -> Self::Output;
}

macro_rules! impl_distance_float {
    ($($t:ty),*) => {
        $(
            impl Distance for $t {
                type Output = $t;
                fn distance(&self, other: &Self) -> Self::Output {
                    (self - other).abs()
                }
            }
        )*
    };
}

macro_rules! impl_distance_int {
    ($($t:ty),*) => {
        $(
            impl Distance for $t {
                type Output = $t;
                fn distance(&self, other: &Self) -> Self::Output {
                    (self - other).unsigned_abs() as $t
                }
            }
        )*
    };
}

macro_rules! impl_distance_unsigned {
    ($($t:ty),*) => {
        $(
            impl Distance for $t {
                type Output = $t;
                fn distance(&self, other: &Self) -> Self::Output {
                    self.abs_diff(*other)
                }
            }
        )*
    };
}

impl_distance_float!(f32, f64);
impl_distance_int!(i8, i16, i32, i64, i128, isize);
impl_distance_unsigned!(u8, u16, u32, u64, u128, usize);

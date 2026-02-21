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
                    (self - other).abs()
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

/// Compute the midpoint of two values (used for coarsening in FastDTW).
pub trait Midpoint {
    fn midpoint(&self, other: &Self) -> Self;
}

macro_rules! impl_midpoint_float {
    ($($t:ty),*) => {
        $(
            impl Midpoint for $t {
                fn midpoint(&self, other: &Self) -> Self {
                    (self + other) / 2.0
                }
            }
        )*
    };
}

macro_rules! impl_midpoint_int {
    ($($t:ty),*) => {
        $(
            impl Midpoint for $t {
                fn midpoint(&self, other: &Self) -> Self {
                    (self + other) / 2
                }
            }
        )*
    };
}

impl_midpoint_float!(f32, f64);
impl_midpoint_int!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

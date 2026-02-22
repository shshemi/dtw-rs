/// The result of a DTW computation, providing the warping distance and path.
///
/// All four algorithm result types (`DtwSolution`, `SakoeChibaSolution`,
/// `ItakuraParallelogramSolution`, `FastDtwSolution`) implement this trait.
pub trait Solution<D> {
    /// Returns the accumulated warping distance between the two input sequences.
    fn distance(&self) -> D;

    /// Returns the optimal warping path as a list of index pairs.
    ///
    /// Each element `(i, j)` in the returned vector represents a match between
    /// index `i` in the first sequence and index `j` in the second sequence.
    /// The path starts at `(0, 0)` and ends at `(len_x - 1, len_y - 1)`.
    fn path(&self) -> Vec<(usize, usize)>;
}

/// A distance metric between two values of the same type.
///
/// This trait is used by the default (non-`_with_distance`) algorithm functions
/// to compute element-wise distances. Built-in implementations are provided for:
///
/// - **Floating-point types** (`f32`, `f64`) — absolute difference
/// - **Signed integers** (`i8`, `i16`, `i32`, `i64`, `i128`, `isize`) — absolute difference
/// - **Unsigned integers** (`u8`, `u16`, `u32`, `u64`, `u128`, `usize`) — absolute difference
///
/// Implement this trait for custom types, or use the `_with_distance` variants
/// to supply a closure instead.
pub trait Distance {
    /// The type returned by the distance computation.
    type Output;
    /// Returns the distance between `self` and `other`.
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

/// Computes the midpoint of two values, used for coarsening sequences in [`fastdtw`](crate::fastdtw).
///
/// When FastDTW recursively halves the input sequences, it averages adjacent
/// pairs of elements using this trait. Built-in implementations are provided for
/// all standard numeric types (`f32`, `f64`, `i8`–`i128`, `u8`–`u128`, `isize`, `usize`).
pub trait Midpoint {
    /// Returns the midpoint (average) of `self` and `other`.
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

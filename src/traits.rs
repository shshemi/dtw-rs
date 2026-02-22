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

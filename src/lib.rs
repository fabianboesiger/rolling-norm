#[cfg(test)]
mod tests;

use num_traits::real::Real;
use std::ops::Index;

/// Implements a ring buffer of length N
/// that continuously and efficiently computes
/// useful information such as mean and variance, or the
/// minimum and maximum values.
///
/// All operations, except initialization, take time O(1).
///
/// The buffer can be accessed using indexing, where
/// rolling[0] is the latest value, rolling[1], is the
/// value inserted before, and so on.
#[derive(Clone, Copy, Debug)]
pub struct Rolling<R: Real, const N: usize> {
    buf: [R; N],
    offset: usize,
    n: R,
    mean: R,
    variance: R,
}

impl<R: Real, const N: usize> Rolling<R, N> {
    /// Create a new rolling series.
    /// All values are initialized to zero.
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a new value into the rolling series.
    pub fn insert(&mut self, new: R) {
        // Increment index, get and replace old value.
        self.offset = (self.offset + 1) % N;
        let old = self.buf[self.offset];
        self.buf[self.offset] = new;

        // Compute new mean.
        let old_mean = self.mean;
        self.mean = self.mean + (new - old) / self.n;

        // Compute new variance.
        self.variance = self.variance + (new - old) * (new - self.mean + old - old_mean) / self.n;
    }

    /// Returns the mean, or average.
    pub fn mean(&self) -> R {
        self.mean
    }

    /// Returns the variance.
    pub fn var(&self) -> R {
        self.variance
    }

    /// Returns the standard derivation.
    pub fn stdev(&self) -> R {
        self.var().sqrt()
    }

    /// Returns the latest value.
    pub fn curr(&self) -> R {
        self.buf[self.offset]
    }

    /// Returns the latest value, normalized.
    /// If the standard derivation is zero, return zero.
    pub fn norm(&self) -> R {
        if self.stdev() == R::zero() {
            R::zero()
        } else {
            (self.curr() - self.mean()) / self.stdev()
        }
    }
}

impl<R: Real, const N: usize> From<[R; N]> for Rolling<R, N> {
    fn from(buf: [R; N]) -> Self {
        let n = R::from(N).expect("Couldn't convert N to type R.");

        let mean = buf.iter().fold(R::zero(), |acc, &x| acc + x) / n;

        let variance = buf
            .iter()
            .map(|&x| (x - mean).powi(2))
            .fold(R::zero(), |acc, x| acc + x)
            / n;

        Rolling {
            buf,
            offset: buf.len() - 1,
            n,
            mean,
            variance,
        }
    }
}

impl<R: Real, const N: usize> Index<usize> for Rolling<R, N> {
    type Output = R;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < N, "Index wasn't less than N.");
        let index = (N + self.offset - index) % N;
        &self.buf[index]
    }
}

impl<R: Real, const N: usize> Default for Rolling<R, N> {
    fn default() -> Self {
        Self::from([R::zero(); N])
    }
}
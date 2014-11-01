use std::collections::HashSet;
use std::default::Default;
use std::hash::Hash;

use {Commute, Partial};
use super::sorted::{mode_on_sorted, median_on_sorted};

/// Compute the exact median on a stream of data.
///
/// (This has time complexity `O(nlogn)` and space complexity `O(n)`.)
pub fn median<T: PartialOrd + ToPrimitive, I: Iterator<T>>
             (mut it: I) -> Option<f64> {
    it.collect::<Unsorted<T>>().median()
}

/// Compute the exact mode on a stream of data.
///
/// (This has time complexity `O(nlogn)` and space complexity `O(n)`.)
///
/// If the data does not have a mode, then `None` is returned.
pub fn mode<T: PartialOrd + Clone, I: Iterator<T>>(mut it: I) -> Option<T> {
    it.collect::<Unsorted<T>>().mode()
}

/// A commutative data structure for lazily sorted sequences of data.
/// 
/// The sort does not occur until statistics need to be computed.
///
/// Note that this works on types that do not define a total ordering like
/// `f32` and `f64`. When an ordering is not defined, an arbitrary order
/// is returned.
#[deriving(Clone)]
pub struct Unsorted<T> {
    data: Vec<Partial<T>>,
    sorted: bool,
}

impl<T: PartialOrd> Unsorted<T> {
    /// Create initial empty state.
    pub fn new() -> Unsorted<T> {
        Default::default()
    }

    /// Add a new element to the set.
    pub fn add(&mut self, v: T) {
        self.dirtied();
        self.data.push(Partial(v))
    }

    fn sort(&mut self) {
        if !self.sorted {
            self.data.sort();
        }
    }

    fn dirtied(&mut self) {
        self.sorted = false;
    }
}

impl<T: PartialOrd + Eq + Hash> Unsorted<T> {
    pub fn cardinality(&self) -> uint {
        let mut set = HashSet::with_capacity(self.len());
        for x in self.data.iter() {
            set.insert(x);
        }
        set.len()
    }
}

impl<T: PartialOrd + Clone> Unsorted<T> {
    /// Returns the mode of the data.
    pub fn mode(&mut self) -> Option<T> {
        self.sort();
        mode_on_sorted(self.data.iter()).map(|p| p.0.clone())
    }
}

impl<T: PartialOrd + ToPrimitive> Unsorted<T> {
    /// Returns the median of the data.
    pub fn median(&mut self) -> Option<f64> {
        self.sort();
        median_on_sorted(self.data[])
    }
}

impl<T: PartialOrd> Commute for Unsorted<T> {
    fn merge(&mut self, v: Unsorted<T>) {
        self.dirtied();
        self.data.extend(v.data.into_iter());
    }
}

impl<T: PartialOrd> Default for Unsorted<T> {
    fn default() -> Unsorted<T> {
        Unsorted {
            data: Vec::with_capacity(1000),
            sorted: true,
        }
    }
}

impl<T: PartialOrd> Collection for Unsorted<T> {
    fn len(&self) -> uint { self.data.len() }
}

impl<T: PartialOrd> Mutable for Unsorted<T> {
    fn clear(&mut self) { self.sorted = true; self.data.clear(); }
}

impl<T: PartialOrd> FromIterator<T> for Unsorted<T> {
    fn from_iter<I: Iterator<T>>(it: I) -> Unsorted<T> {
        let mut v = Unsorted::new();
        v.extend(it);
        v
    }
}

impl<T: PartialOrd> Extendable<T> for Unsorted<T> {
    fn extend<I: Iterator<T>>(&mut self, it: I) {
        self.dirtied();
        self.data.extend(it.map(Partial))
    }
}

#[cfg(test)]
mod test {
    use super::{median, mode};

    #[test]
    fn median_stream() {
        assert_eq!(median(vec![3u, 5, 7, 9].into_iter()), Some(6.0));
        assert_eq!(median(vec![3u, 5, 7].into_iter()), Some(5.0));
    }

    #[test]
    fn mode_stream() {
        assert_eq!(mode(vec![3u, 5, 7, 9].into_iter()), None);
        assert_eq!(mode(vec![3u, 3, 3, 3].into_iter()), Some(3));
        assert_eq!(mode(vec![3u, 3, 3, 4].into_iter()), Some(3));
        assert_eq!(mode(vec![4u, 3, 3, 3].into_iter()), Some(3));
        assert_eq!(mode(vec![1u, 1, 2, 3, 3].into_iter()), None);
    }

    #[test]
    fn median_floats() {
        assert_eq!(median(vec![3.0f64, 5.0, 7.0, 9.0].into_iter()), Some(6.0));
        assert_eq!(median(vec![3.0f64, 5.0, 7.0].into_iter()), Some(5.0));
    }

    #[test]
    fn mode_floats() {
        assert_eq!(mode(vec![3.0f64, 5.0, 7.0, 9.0].into_iter()), None);
        assert_eq!(mode(vec![3.0f64, 3.0, 3.0, 3.0].into_iter()), Some(3.0));
        assert_eq!(mode(vec![3.0f64, 3.0, 3.0, 4.0].into_iter()), Some(3.0));
        assert_eq!(mode(vec![4.0f64, 3.0, 3.0, 3.0].into_iter()), Some(3.0));
        assert_eq!(mode(vec![1.0f64, 1.0, 2.0, 3.0, 3.0].into_iter()), None);
    }
}
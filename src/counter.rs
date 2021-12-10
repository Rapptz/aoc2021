use std::borrow::Borrow;
use std::collections::hash_map::{IntoIter, Iter, IterMut, Keys, Values, ValuesMut};
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::{Index, IndexMut};

/// A counter modeled after Python's.
#[derive(Default)]
pub struct Counter<K> {
    // generalising over i64 is too much effort for too little gain
    data: HashMap<K, i64>,
}

impl<K> Counter<K> {
    /// Creates an empty counter.
    #[inline]
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    /// Returns the number of elements in the counter.
    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns `true` if the counter has no elements.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Computes a sum over the counter.
    #[inline]
    pub fn total(&self) -> i64 {
        self.data.values().sum()
    }

    /// Returns the inner underlying HashMap storage.
    #[inline]
    pub fn into_inner(self) -> HashMap<K, i64> {
        self.data
    }

    /// Returns an iterator over the values
    #[inline]
    pub fn values<'a>(&'a self) -> Values<'a, K, i64> {
        self.data.values()
    }

    /// Returns an iterator over the keys
    #[inline]
    pub fn keys<'a>(&'a self) -> Keys<'a, K, i64> {
        self.data.keys()
    }

    /// Returns an iterator over the `(key, value)` pairs.
    #[inline]
    pub fn iter<'a>(&'a self) -> Iter<'a, K, i64> {
        self.data.iter()
    }

    /// Returns an iterator over the values mutably
    #[inline]
    pub fn values_mut<'a>(&'a mut self) -> ValuesMut<'a, K, i64> {
        self.data.values_mut()
    }

    /// Returns an iterator over the `(key, value)` pairs mutably.
    #[inline]
    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, K, i64> {
        self.data.iter_mut()
    }

    /// Returns a copy to the internal value corresponding to the key.
    ///
    /// If the key is not found then [`None`] is returned.
    #[inline]
    pub fn get<'a, Q>(&'a self, key: &Q) -> Option<i64>
    where
        K: Eq + Hash + Borrow<Q>,
        Q: ?Sized + Eq + Hash,
    {
        self.data.get(key).copied()
    }

    /// Returns a mutable reference to the element pointed by the key.
    ///
    /// The key must be [`Copy`] since it may end up inserting a default value
    /// before returning the reference.
    ///
    /// Note that this is different from [`HashMap::entry`].
    #[inline]
    pub fn entry<'a>(&'a mut self, key: K) -> &'a mut i64
    where
        K: Eq + Hash + Copy,
    {
        self.data.entry(key).or_default()
    }

    /// Removes a key from the counter.
    #[inline]
    pub fn remove<Q>(&mut self, key: &Q) -> Option<i64>
    where
        K: Eq + Hash + Borrow<Q>,
        Q: ?Sized + Eq + Hash,
    {
        self.data.remove(key)
    }

    /// Clears the counter but retains memory
    #[inline]
    pub fn clear(&mut self) {
        self.data.clear()
    }

    /// Returns the most common elements in the counter.
    ///
    /// These values are sorted in descending order.
    pub fn most_common(&self) -> Vec<(K, i64)>
    where
        K: Clone,
    {
        let mut values: Vec<_> = self.data.iter().map(|(k, v)| (k.clone(), *v)).collect();
        values.sort_by(|(_, lhs), (_, rhs)| rhs.cmp(lhs));
        values
    }

    /// Returns the N most common elements in the counter.
    ///
    /// These values are sorted in descending order.
    pub fn take_most_common(&self, count: usize) -> Vec<(K, i64)>
    where
        K: Clone,
    {
        let mut values: Vec<_> = self.data.iter().map(|(k, v)| (k.clone(), *v)).collect();
        values.sort_by(|(_, lhs), (_, rhs)| rhs.cmp(lhs));
        values.truncate(count);
        values
    }

    /// Return an iterator over elements repeating each as many times as its count.
    ///
    /// Elements are returned in no particular order.
    /// If an element’s count is less than one, it'll be ignored.
    pub fn elements<'a>(&'a self) -> impl Iterator<Item = &'a K> {
        self.iter()
            .flat_map(|(k, v)| std::iter::repeat(k).take(*v as usize))
    }
}

impl<'a, K> IntoIterator for &'a Counter<K> {
    type Item = (&'a K, &'a i64);
    type IntoIter = Iter<'a, K, i64>;

    #[inline]
    fn into_iter(self) -> Iter<'a, K, i64> {
        self.iter()
    }
}

impl<'a, K> IntoIterator for &'a mut Counter<K> {
    type Item = (&'a K, &'a mut i64);
    type IntoIter = IterMut<'a, K, i64>;

    #[inline]
    fn into_iter(self) -> IterMut<'a, K, i64> {
        self.iter_mut()
    }
}

impl<K> IntoIterator for Counter<K> {
    type Item = (K, i64);
    type IntoIter = IntoIter<K, i64>;

    #[inline]
    fn into_iter(self) -> IntoIter<K, i64> {
        self.data.into_iter()
    }
}

impl<K> FromIterator<K> for Counter<K>
where
    K: Eq + Hash,
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = K>,
    {
        let iter = iter.into_iter();
        let mut data = HashMap::with_capacity(iter.size_hint().0);
        for obj in iter {
            data.entry(obj).and_modify(|f| *f += 1).or_insert(1i64);
        }
        Self { data }
    }
}

impl<K> std::fmt::Debug for Counter<K>
where
    K: std::fmt::Debug,
{
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.data.fmt(f)
    }
}

impl<K> Clone for Counter<K>
where
    K: Clone,
{
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.data.clone_from(&source.data);
    }
}

impl<K> PartialEq for Counter<K>
where
    K: Eq + Hash,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<K> Eq for Counter<K> where K: Eq + Hash {}

impl<K, const N: usize> From<[K; N]> for Counter<K>
where
    K: Eq + Hash,
{
    fn from(arr: [K; N]) -> Self {
        let mut data = HashMap::with_capacity(N);
        for key in arr {
            data.entry(key).and_modify(|f| *f += 1).or_insert(1i64);
        }
        Self { data }
    }
}

impl<K> Extend<K> for Counter<K>
where
    K: Eq + Hash,
{
    fn extend<T: IntoIterator<Item = K>>(&mut self, iter: T) {
        let iter = iter.into_iter();
        for obj in iter {
            self.data.entry(obj).and_modify(|f| *f += 1).or_insert(1i64);
        }
    }
}

macro_rules! impl_primitives {
    ($($e:ty)+) => {
        $(
            impl<K> FromIterator<(K, $e)> for Counter<K>
            where
                K: Eq + Hash,
            {
                fn from_iter<T>(iter: T) -> Self
                where
                    T: IntoIterator<Item = (K, $e)>,
                {
                    let iter = iter.into_iter();
                    let mut data = HashMap::with_capacity(iter.size_hint().0);
                    for (obj, value) in iter {
                        data.entry(obj).and_modify(|f| *f += value as i64).or_insert(value as i64);
                    }
                    Self { data }
                }
            }

            impl<K> Extend<(K, $e)> for Counter<K>
            where
                K: Eq + Hash,
            {
                fn extend<T: IntoIterator<Item = (K, $e)>>(&mut self, iter: T) {
                    let iter = iter.into_iter();
                    for (obj, value) in iter {
                        self.data.entry(obj).and_modify(|f| *f += value as i64).or_insert(value as i64);
                    }
                }
            }

            impl<K, const N: usize> From<[(K, $e); N]> for Counter<K>
            where
                K: Eq + Hash,
            {
                fn from(arr: [(K, $e); N]) -> Self {
                    let mut data = HashMap::with_capacity(N);
                    for (obj, value) in arr {
                        data.entry(obj).and_modify(|f| *f += value as i64).or_insert(value as i64);
                    }
                    Self { data }
                }
            }
        )+
    };
}

// usize and u64 aren't provided since they can silently truncate
impl_primitives!(i64 i32 u32 i16 u16 i8 u8 isize);

impl<K, Q> Index<&Q> for Counter<K>
where
    K: Eq + Hash + Borrow<Q>,
    Q: ?Sized + Eq + Hash,
{
    type Output = i64;

    fn index(&self, index: &Q) -> &Self::Output {
        self.data.get(index).unwrap_or(&0)
    }
}

impl<K, Q> IndexMut<&Q> for Counter<K>
where
    K: Eq + Hash + Borrow<Q>,
    Q: ?Sized + Eq + Hash,
{
    fn index_mut(&mut self, index: &Q) -> &mut Self::Output {
        self.data.get_mut(index).expect("key not found")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_iter() {
        let counter: Counter<_> = "abcaba".chars().collect();
        assert_eq!(counter.len(), 3);
        assert_eq!(counter.total(), 6);
    }

    #[test]
    fn test_from_iter_tuple() {
        let counter: Counter<char> = [('a', 2), ('b', 3), ('c', 1), ('b', 5)]
            .iter()
            .copied()
            .collect();
        assert_eq!(counter.len(), 3);
        assert_eq!(counter.total(), 11);
    }

    #[test]
    fn test_from_keys_array() {
        let counter = Counter::from(['a', 'b', 'c', 'a', 'b', 'a']);
        assert_eq!(counter.len(), 3);
        assert_eq!(counter.total(), 6);
    }

    #[test]
    fn test_from_array_tuple() {
        let counter = Counter::<char>::from([('a', 2), ('b', 3), ('c', 1), ('b', 5)]);
        assert_eq!(counter.len(), 3);
        assert_eq!(counter.total(), 11);
    }

    #[test]
    fn test_basics() {
        let mut counter: Counter<_> = "abcaba".chars().collect();
        assert_eq!(counter.len(), 3);
        assert_eq!(counter.total(), 6);
        assert!(!counter.is_empty());
        assert_eq!(
            &counter,
            &Counter::<char>::from([('a', 3), ('b', 2), ('c', 1)])
        );
        assert!(counter.keys().all(|c| matches!(*c, 'a' | 'b' | 'c')));
        assert!(counter.values().all(|c| matches!(*c, 1 | 2 | 3)));
        assert_eq!(counter[&'b'], 2);
        assert_eq!(counter[&'z'], 0);
        assert_eq!(counter.get(&'b'), Some(2));
        assert_eq!(counter.get(&'z'), None);
        assert_eq!(counter.most_common(), vec![('a', 3), ('b', 2), ('c', 1)]);
        assert_eq!(counter.take_most_common(2), vec![('a', 3), ('b', 2)]);
        let mut elements: Vec<char> = counter.elements().copied().collect();
        elements.sort();
        let elements: String = elements.iter().collect();
        assert_eq!(elements, "aaabbc");

        // mutations
        *counter.entry('a') += 1;
        *counter.entry('b') -= 2;
        assert!(counter.remove(&'c').is_some());
        assert!(counter.remove(&'c').is_none());

        // missing keys
        *counter.entry('d') -= 2;
        *counter.entry('e') = -5;
        *counter.entry('f') += 4;
        assert_eq!(counter[&'a'], 4);
        assert_eq!(counter[&'b'], 0);
        assert_eq!(counter[&'d'], -2);
        assert_eq!(counter[&'e'], -5);
        assert_eq!(counter[&'f'], 4);
        assert_eq!(counter.remove(&'f'), Some(4));

        counter.clear();
        assert!(counter.is_empty());
    }
}

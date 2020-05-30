use std::collections::hash_map::RandomState;
use std::collections::HashMap;

#[derive(Clone)]
pub struct OrderedHashMap<K, V, S = RandomState> {
    base: HashMap<K, V, S>,
    order_list: Vec<K>,
}

impl<K, V> OrderedHashMap<K, V, RandomState> {
    /// Creates an empty `OrderedHashMap`.
    ///
    /// The hash map is initially created with a capacity of 0, so it will not allocate until it
    /// is first inserted into.
    ///
    /// # Examples
    ///
    /// ```
    /// use ordered::OrderedHashMap;
    /// let mut map: OrderedHashMap<&str, i32> = OrderedHashMap::new();
    /// ```
    #[inline]
    pub fn new() -> OrderedHashMap<K, V, RandomState> {
        Default::default()
    }
    /// Creates an empty `OrderedHashMap` with the specified capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use ordered::OrderedHashMap;
    /// let mut map: OrderedHashMap<&str, i32> = OrderedHashMap::with_capacity(10);
    /// ```
    #[inline]
    pub fn with_capacity(capacity: usize) -> OrderedHashMap<K, V, RandomState> {
        OrderedHashMap::with_capacity_and_hasher(capacity, Default::default())
    }
}

impl<K, V, S> OrderedHashMap<K, V, S> {
    /// Creates an empty `OrderedHashMap` which will use the given hash builder to hash
    /// keys.
    ///
    /// The created map has the default initial capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use ordered::OrderedHashMap;
    /// use std::collections::hash_map::RandomState;
    ///
    /// let s = RandomState::new();
    /// let mut map = OrderedHashMap::<u32,u32>::with_hasher(s);
    /// //map.insert(1, 2);
    /// ```
    #[inline]
    pub fn with_hasher(hash_builder: S) -> OrderedHashMap<K, V, S> {
        OrderedHashMap {
            base: HashMap::with_hasher(hash_builder),
            order_list: Vec::<K>::new(),
        }
    }
    /// Creates an empty `OrderedHashMap` with the specified capacity, using `hash_builder`
    /// to hash the keys.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use ordered::OrderedHashMap;
    /// use std::collections::hash_map::RandomState;
    ///
    /// let s = RandomState::new();
    /// let mut map = OrderedHashMap::<u32, u32>::with_capacity_and_hasher(10, s);
    /// //map.insert(1, 2);
    /// ```
    #[inline]
    pub fn with_capacity_and_hasher(capacity: usize, hash_builder: S) -> OrderedHashMap<K, V, S> {
        OrderedHashMap {
            base: HashMap::with_capacity_and_hasher(capacity, hash_builder),
            order_list: Vec::<K>::with_capacity(capacity),
        }
    }

    /// Returns the number of elements the map can hold without reallocating.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// let map: HashMap<i32, i32> = HashMap::with_capacity(100);
    /// assert!(map.capacity() >= 100);
    /// ```
    #[inline]
    pub fn capacity(&self) -> usize {
        self.order_list.capacity()
    }
}

impl<K, V, S> Default for OrderedHashMap<K, V, S>
where
    S: Default,
{
    /// Creates an empty `OrderedHashMap<K, V, S>`, with the `Default` value for the hasher.
    #[inline]
    fn default() -> OrderedHashMap<K, V, S> {
        OrderedHashMap::with_hasher(Default::default())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

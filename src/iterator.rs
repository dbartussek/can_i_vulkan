use itertools::Itertools;
use linked_hash_map::LinkedHashMap;
use std::{
    collections::{BTreeMap, HashMap},
    hash::Hash,
};

pub fn count_key<It, T>(it: It) -> LinkedHashMap<T, usize>
where
    It: IntoIterator<Item = T>,
    T: Hash + Eq,
{
    let mut map = HashMap::<T, usize>::new();

    for t in it {
        *map.entry(t).or_default() += 1;
    }

    map.into_iter().sorted_by(|a, b| b.1.cmp(&a.1)).collect()
}

pub fn group_by<It, F, Key>(it: It, mut f: F) -> BTreeMap<Key, Vec<It::Item>>
where
    It: IntoIterator,
    F: FnMut(&It::Item) -> Key,
    Key: Ord + Clone,
{
    it.into_iter()
        .map(|item| {
            let key = f(&item);
            (key, item)
        })
        .sorted_by_key(|(k, _)| k.clone())
        .group_by(|(k, _)| k.clone())
        .into_iter()
        .map(|(key, group)| (key, group.map(|(_, value)| value).collect()))
        .collect()
}

use core::hash::{BuildHasher, Hash};

use crate::{
    binary_heap::{BinaryHeapInner, Kind as BinaryHeapKind},
    deque::DequeInner,
    histbuf::HistoryBufferInner,
    linear_map::LinearMapInner,
    storage::Storage,
    string::StringInner,
    vec::VecInner,
    IndexMap, IndexSet,
};
use serde::ser::{Serialize, SerializeMap, SerializeSeq, Serializer};

// Sequential containers

impl<T, KIND, S> Serialize for BinaryHeapInner<T, KIND, S>
where
    T: Ord + Serialize,
    KIND: BinaryHeapKind,
    S: Storage,
{
    fn serialize<SER>(&self, serializer: SER) -> Result<SER::Ok, SER::Error>
    where
        SER: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for element in self {
            seq.serialize_element(element)?;
        }
        seq.end()
    }
}

impl<T, S, const N: usize> Serialize for IndexSet<T, S, N>
where
    T: Eq + Hash + Serialize,
    S: BuildHasher,
{
    fn serialize<SER>(&self, serializer: SER) -> Result<SER::Ok, SER::Error>
    where
        SER: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for element in self {
            seq.serialize_element(element)?;
        }
        seq.end()
    }
}

impl<T, St: Storage> Serialize for VecInner<T, St>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for element in self {
            seq.serialize_element(element)?;
        }
        seq.end()
    }
}

impl<T, S: Storage> Serialize for DequeInner<T, S>
where
    T: Serialize,
{
    fn serialize<SER>(&self, serializer: SER) -> Result<SER::Ok, SER::Error>
    where
        SER: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.storage_len()))?;
        for element in self {
            seq.serialize_element(element)?;
        }
        seq.end()
    }
}

impl<T, S: Storage> Serialize for HistoryBufferInner<T, S>
where
    T: Serialize,
{
    fn serialize<SER>(&self, serializer: SER) -> Result<SER::Ok, SER::Error>
    where
        SER: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for element in self.oldest_ordered() {
            seq.serialize_element(element)?;
        }
        seq.end()
    }
}

// Dictionaries

impl<K, V, S, const N: usize> Serialize for IndexMap<K, V, S, N>
where
    K: Eq + Hash + Serialize,
    S: BuildHasher,
    V: Serialize,
{
    fn serialize<SER>(&self, serializer: SER) -> Result<SER::Ok, SER::Error>
    where
        SER: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.len()))?;
        for (k, v) in self {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

impl<K, V, S: Storage> Serialize for LinearMapInner<K, V, S>
where
    K: Eq + Serialize,
    V: Serialize,
{
    fn serialize<SER>(&self, serializer: SER) -> Result<SER::Ok, SER::Error>
    where
        SER: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.len()))?;
        for (k, v) in self {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

// String containers

impl<S: Storage> Serialize for StringInner<S> {
    fn serialize<SER>(&self, serializer: SER) -> Result<SER::Ok, SER::Error>
    where
        SER: Serializer,
    {
        serializer.serialize_str(&*self)
    }
}

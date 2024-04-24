use std::collections::BTreeMap;

pub trait IntoTupleVec<K, V> {
    fn into_tuple_vec(self) -> Vec<(K, V)>;
}

impl<K, V> IntoTupleVec<K, V> for BTreeMap<K, V> {
    fn into_tuple_vec(self) -> Vec<(K, V)> {
        self.into_iter().collect::<Vec<(K, V)>>()
    }
}

use crate::{Dim, IntoIndex};

pub trait NVec<N: Dim, T> {
    fn at<Idx: IntoIndex<N>>(&self, index: Idx) -> T;

    fn can_get_at<Idx: IntoIndex<N>>(&self, index: Idx) -> bool;

    fn can_get_at_all<Idx: IntoIndex<N>>(&self, indices: impl IntoIterator<Item = Idx>) -> bool {
        indices.into_iter().all(|index| self.can_get_at(index))
    }
}

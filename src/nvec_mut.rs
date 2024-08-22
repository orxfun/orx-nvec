use crate::{Dim, IntoIndex, NVec};

pub trait NVecMut<N: Dim, T>: NVec<N, T> {
    fn set<Idx: IntoIndex<N>>(&mut self, index: Idx, value: T);

    fn can_set_at<Idx: IntoIndex<N>>(&self, index: Idx) -> bool;

    fn can_set_at_all<Idx: IntoIndex<N>>(&self, indices: impl IntoIterator<Item = Idx>) -> bool {
        indices.into_iter().all(|index| self.can_set_at(index))
    }

    fn can_get_set_at_all<Idx: IntoIndex<N>>(
        &self,
        indices: impl IntoIterator<Item = Idx>,
    ) -> bool {
        indices
            .into_iter()
            .all(|index| self.can_get_at(index) && self.can_set_at(index))
    }
}

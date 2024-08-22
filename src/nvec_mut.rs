use crate::{Dim, IntoIndex, NVec};

pub trait NVecMut<N: Dim, T>: NVec<N, T> {
    fn set<Idx: IntoIndex<N>>(&mut self, index: Idx, value: T);

    fn can_set_at<Idx: IntoIndex<N>>(&self, index: Idx) -> bool;
    //  {true}
}

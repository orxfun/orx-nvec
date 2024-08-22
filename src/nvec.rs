use crate::{Dim, IntoIndex};

pub trait NVec<N: Dim, T> {
    fn at<Idx: IntoIndex<N>>(&self, index: Idx) -> T;

    fn is_index_valid<Idx: IntoIndex<N>>(&self, index: Idx) -> bool;
}

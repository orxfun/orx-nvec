use crate::{dimensions::*, NVec, NVecMut};

impl<T, E> NVec<D2, T> for &[E]
where
    E: NVec<<D2 as Dim>::PREVIOUS, T>,
{
    fn at<Idx: IntoIndex<D2>>(&self, index: Idx) -> T {
        let (i, index) = index.split();
        self[i].at(index)
    }

    fn can_get_at<Idx: IntoIndex<D2>>(&self, index: Idx) -> bool {
        let (i, index) = index.split();
        i < self.len() && self[i].can_get_at(index)
    }
}

impl<T, E> NVec<D2, T> for &mut [E]
where
    E: NVec<<D2 as Dim>::PREVIOUS, T>,
{
    fn at<Idx: IntoIndex<D2>>(&self, index: Idx) -> T {
        let (i, index) = index.split();
        self[i].at(index)
    }

    fn can_get_at<Idx: IntoIndex<D2>>(&self, index: Idx) -> bool {
        let (i, index) = index.split();
        i < self.len() && self[i].can_get_at(index)
    }
}

impl<T, E> NVecMut<D2, T> for &mut [E]
where
    E: NVecMut<<D2 as Dim>::PREVIOUS, T>,
{
    fn set<Idx: IntoIndex<D2>>(&mut self, index: Idx, value: T) {
        let (i, index) = index.split();
        self[i].set(index, value)
    }

    fn can_set_at<Idx: IntoIndex<D2>>(&self, index: Idx) -> bool {
        self.can_get_at(index)
    }
}

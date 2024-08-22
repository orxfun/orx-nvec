use crate::{dimensions::*, NVec, NVecMut};

macro_rules! implement {
    ($dim:tt) => {
        impl<T, E> NVec<$dim, T> for &[E]
        where
            E: NVec<<$dim as Dim>::PREVIOUS, T>,
        {
            fn at<Idx: IntoIndex<$dim>>(&self, index: Idx) -> T {
                let (i, index) = index.split();
                self[i].at(index)
            }

            fn can_get_at<Idx: IntoIndex<$dim>>(&self, index: Idx) -> bool {
                let (i, index) = index.split();
                i < self.len() && self[i].can_get_at(index)
            }
        }

        impl<T, E> NVec<$dim, T> for &mut [E]
        where
            E: NVec<<$dim as Dim>::PREVIOUS, T>,
        {
            fn at<Idx: IntoIndex<$dim>>(&self, index: Idx) -> T {
                let (i, index) = index.split();
                self[i].at(index)
            }

            fn can_get_at<Idx: IntoIndex<$dim>>(&self, index: Idx) -> bool {
                let (i, index) = index.split();
                i < self.len() && self[i].can_get_at(index)
            }
        }

        impl<T, E> NVecMut<$dim, T> for &mut [E]
        where
            E: NVecMut<<$dim as Dim>::PREVIOUS, T>,
        {
            fn set<Idx: IntoIndex<$dim>>(&mut self, index: Idx, value: T) {
                let (i, index) = index.split();
                self[i].set(index, value)
            }

            fn can_set_at<Idx: IntoIndex<$dim>>(&self, index: Idx) -> bool {
                self.can_get_at(index)
            }
        }
    };
}

implement!(D3);
implement!(D4);
implement!(D5);
implement!(D6);
implement!(D7);
implement!(D8);

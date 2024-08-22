use crate::{Dim, IntoIndex, NVec, NVecMut};
use core::marker::PhantomData;

pub enum HookTrigger {
    OnAt,
    OnSet,
}

pub struct Hooked<N, T, V, H>
where
    N: Dim,
    H: Fn(N::Idx, &T),
{
    inner: V,
    hook: H,
    hook_trigger: HookTrigger,
    phantom: PhantomData<(N, T)>,
}

// nvecs

impl<N, T, V, H> NVec<N, T> for Hooked<N, T, V, H>
where
    N: Dim,
    V: NVec<N, T>,
    H: Fn(N::Idx, &T),
{
    fn at<Idx: IntoIndex<N>>(&self, index: Idx) -> T {
        let idx = index.into_index();
        let value = self.inner.at(index);
        if let HookTrigger::OnAt = self.hook_trigger {
            (self.hook)(idx, &value);
        }
        value
    }

    fn can_get_at<Idx: IntoIndex<N>>(&self, index: Idx) -> bool {
        self.inner.can_get_at(index)
    }
}

impl<N, T, V, H> NVecMut<N, T> for Hooked<N, T, V, H>
where
    N: Dim,
    V: NVecMut<N, T>,
    H: Fn(N::Idx, &T),
{
    fn set<Idx: IntoIndex<N>>(&mut self, index: Idx, value: T) {
        let idx = index.into_index();
        if let HookTrigger::OnSet = self.hook_trigger {
            (self.hook)(idx, &value);
        }
        self.inner.set(idx, value)
    }

    fn can_set_at<Idx: IntoIndex<N>>(&self, index: Idx) -> bool {
        self.inner.can_set_at(index)
    }
}

// into

pub trait IntoHooked<N, T, H>
where
    N: Dim,
    Self: NVec<N, T> + Sized,
    H: Fn(N::Idx, &T),
{
    fn hooked_on_at(self, hook: H) -> Hooked<N, T, Self, H>;

    fn hooked_on_set(self, hook: H) -> Hooked<N, T, Self, H>;
}

impl<N, T, V, H> IntoHooked<N, T, H> for V
where
    N: Dim,
    V: NVec<N, T>,
    H: Fn(N::Idx, &T),
{
    fn hooked_on_at(self, hook: H) -> Hooked<N, T, V, H> {
        Hooked {
            inner: self,
            hook,
            hook_trigger: HookTrigger::OnAt,
            phantom: Default::default(),
        }
    }

    fn hooked_on_set(self, hook: H) -> Hooked<N, T, V, H> {
        Hooked {
            inner: self,
            hook,
            hook_trigger: HookTrigger::OnSet,
            phantom: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use core::cell::RefCell;
    use std::collections::BTreeMap;

    fn run<V: NVec<D2, char>>(vec: V) {
        let _ = vec.at([2, 1]);
        let _ = vec.at([0, 0]);
        let _ = vec.at([2, 3]);
    }

    fn log([i, j]: [usize; 2], value: &char) {
        println!("x[{},{}] = {:?}", i, j, value)
    }

    #[test]
    fn log_hook() {
        let vec_vec = vec![vec!['1', '2'], vec![], vec!['3', '4', '5', '6']];
        run(vec_vec.hooked_on_at(log));

        let map_vec: BTreeMap<usize, Vec<char>> = [
            (0, vec!['1']),
            (4, vec!['2', '3', '4', '5', '6']),
            (2, vec!['7', '8', '9', '0']),
        ]
        .into_iter()
        .collect();
        run((&map_vec).hooked_on_at(log));
    }

    #[test]
    fn recursively_hooked() {
        let num_called = RefCell::new(0);
        let hook_increment = |_: [usize; 2], _: &char| *num_called.borrow_mut() += 1;

        let vec_vec = vec![vec!['1', '2'], vec![], vec!['3', '4', '5', '6']];
        run(vec_vec.hooked_on_at(log).hooked_on_at(hook_increment));

        assert_eq!(*num_called.borrow(), 3);
    }
}

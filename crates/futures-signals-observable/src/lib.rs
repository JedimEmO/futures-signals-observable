use futures::Stream;
use futures_signals::signal::SignalExt;
use futures_signals::signal::{Mutable, Signal};
use futures_signals::signal_map::MutableBTreeMap;
use futures_signals::signal_vec::{MutableVec, SignalVecExt};

pub trait Observable {
    fn changed(&self) -> impl Stream<Item = ()>;
}

impl<T> Observable for Mutable<T> {
    fn changed(&self) -> impl Stream<Item = ()> {
        self.signal_ref(|_| ()).to_stream()
    }
}

impl<K: Ord + Clone, T: Clone> Observable for MutableBTreeMap<K, T> {
    fn changed(&self) -> impl Stream<Item = ()> {
        self.signal_vec_keys()
            .map(|_| ())
            .to_signal_cloned()
            .map(|_| ())
            .to_stream()
    }
}

impl<T: Clone> Observable for MutableVec<T> {
    fn changed(&self) -> impl Stream<Item = ()> {
        self.signal_vec_cloned()
            .map(|_| ())
            .to_signal_cloned()
            .map(|_| ())
            .to_stream()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt;

    struct TestA {
        a: Mutable<u32>,
        b: Mutable<String>,
    }

    impl Observable for TestA {
        fn changed(&self) -> impl Stream<Item = ()> {
            futures::stream::select_all([self.a.changed().boxed(), self.b.changed().boxed()])
        }
    }

    #[test]
    fn basic_observable() {}
}

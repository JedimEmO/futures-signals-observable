use futures::{Stream, StreamExt};
use futures::stream::{empty, iter, select, select_all};
use futures_signals::signal::{always, SignalExt};
use futures_signals::signal::{Mutable, Signal};
use futures_signals::signal_map::MutableBTreeMap;
use futures_signals::signal_vec::{from_stream, MutableVec, SignalVecExt};

#[cfg(feature = "futures-signals-observable")]
#[doc(hidden)]
pub use futures_signals::{self, *};

pub trait Observable {
    fn changed(&self) -> impl Stream<Item=()> + Send + 'static;
}

impl<T: Observable + Send + Sync + Clone + 'static> Observable for Mutable<T> {
    fn changed(&self) -> impl Stream<Item=()> + Send + 'static {
        let a = self.signal_cloned()
            .to_stream()
            .map(|v| {
                v.changed().boxed()
            })
            .flatten();

        let b = self.signal_cloned()
            .to_stream().map(|_| ());

        select(a, b)
    }
}

impl<K: Ord + Clone + Send + 'static, T: Clone + Send + Observable + 'static> Observable for MutableBTreeMap<K, T> {
    fn changed(&self) -> impl Stream<Item=()> + Send + 'static {
        self.entries_cloned()
            .to_signal_cloned()
            .map(|v| {
                iter(v.into_iter().map(|(_k, v)| v.changed().boxed()))
            }).to_stream()
            .flatten()
            .flatten()
    }
}

impl<T: Observable + Clone + Send + 'static> Observable for MutableVec<T> {
    fn changed(&self) -> impl Stream<Item=()> + Send + 'static {
        self.signal_vec_cloned()
            .to_signal_cloned()
            .map(|v| iter(v.into_iter().map(|v| v.changed().boxed())))
            .to_stream().flatten().flatten()
    }
}


impl Observable for String {
    fn changed(&self) -> impl Stream<Item=()> + Send + 'static {
        empty()
    }
}

impl Observable for &str {
    fn changed(&self) -> impl Stream<Item=()> + Send + 'static {
        empty()
    }
}


impl Observable for i32 {
    fn changed(&self) -> impl Stream<Item=()> + Send + 'static {
        empty()
    }
}

impl Observable for u32 {
    fn changed(&self) -> impl Stream<Item=()> + Send + 'static {
        empty()
    }
}

impl Observable for i64 {
    fn changed(&self) -> impl Stream<Item=()> + Send + 'static {
        empty()
    }
}

impl Observable for u64 {
    fn changed(&self) -> impl Stream<Item=()> + Send + 'static {
        empty()
    }
}

impl Observable for i128 {
    fn changed(&self) -> impl Stream<Item=()> + Send + 'static {
        empty()
    }
}

impl Observable for u128 {
    fn changed(&self) -> impl Stream<Item=()> + Send + 'static {
        empty()
    }
}

impl Observable for isize {
    fn changed(&self) -> impl Stream<Item=()> + Send + 'static {
        empty()
    }
}

impl Observable for usize {
    fn changed(&self) -> impl Stream<Item=()> + Send + 'static {
        empty()
    }
}

impl Observable for f32 {
    fn changed(&self) -> impl Stream<Item=()> + Send + 'static {
        empty()
    }
}

impl Observable for f64 {
    fn changed(&self) -> impl Stream<Item=()> + Send + 'static {
        empty()
    }
}

impl<T> Observable for Option<T> {
    fn changed(&self) -> impl Stream<Item=()> + Send + 'static {
        empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt;
    use tokio::task::yield_now;

    struct TestA {
        a: Mutable<u32>,
        b: Mutable<String>,
    }

    impl Observable for TestA {
        fn changed(&self) -> impl Stream<Item=()> + Send + 'static {
            futures::stream::select_all([
                self.a.changed().boxed(),
                self.b.changed().boxed()
            ])
        }
    }

    #[tokio::test]
    async fn basic_observable() {
        let a = TestA {
            a: Mutable::new(0),
            b: Mutable::new("".to_string()),
        };

        let mut changes = a.changed();
        let change_count = Mutable::new(0);
        let change_count_cloned = change_count.clone();

        tokio::spawn(async move {
            changes.for_each(|_| {
                println!("change detected");
                change_count_cloned.set(change_count_cloned.get() + 1);
                async move {}
            }).await;
        });

        while change_count.get() < 10 {
            println!("change count: {}", change_count.get());
            a.a.set(a.a.get() + 1);
            yield_now().await;
        }
    }
}


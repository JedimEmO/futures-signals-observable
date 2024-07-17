use futures::StreamExt;
use futures_signals::signal::Mutable;
use futures_signals::signal_map::MutableBTreeMap;
use futures_signals::signal_vec::MutableVec;
use futures_signals_observable::Observable;
use futures_signals_observable_derive::Observable;
use tokio::task::yield_now;

#[derive(Observable, Default)]
struct TestB {
    a: Mutable<u32>,
    b: MutableVec<String>,
}

#[derive(Observable, Default)]
struct TestA {
    a: MutableBTreeMap<String, u32>,
    b: Mutable<String>,
    c: TestB,
}

#[tokio::test]
async fn test_nested_observable() {
    let mut a = Box::leak(Box::new(TestA::default()));
    let change_count = Mutable::new(0);
    let change_count_cloned = change_count.clone();

    let changes = a.changed();

    tokio::spawn(async move {
        changes
            .for_each(|_| {
                change_count_cloned.set(change_count_cloned.get() + 1);
                async move {}
            })
            .await;
    });

    a.c.b.lock_mut().push_cloned("hi there".to_string());

    while change_count.get() < 10 {
        a.c.a.set(a.c.a.get() + 1);
        yield_now().await;
    }
}

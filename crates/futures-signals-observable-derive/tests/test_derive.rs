#[macro_use]
extern crate futures_signals_observable_derive;

use futures::StreamExt;
use futures_signals::signal::Mutable;
use futures_signals::signal_map::MutableBTreeMap;
use futures_signals::signal_vec::MutableVec;
use futures_signals_observable::*;
use std::sync::Arc;
use tokio::task::yield_now;

#[derive(Observable, Default)]
struct TestB {
    a: Mutable<u32>,
    b: MutableVec<String>,
    c: MutableVec<Mutable<i32>>,
}

#[derive(Observable, Default)]
struct TestA {
    a: MutableBTreeMap<String, u32>,
    b: Mutable<String>,
    #[shallow]
    unobservable: Mutable<UnObservable>,
    c: TestB,
    enum_: Mutable<SomeEnum>
}

#[derive(Clone, Default)]
struct UnObservable {}

    #[derive(Observable, Clone)]
    enum SomeEnum {
        Struct { a: Mutable<i32>, b: String, #[shallow] shallow: Mutable<UnObservable>},
        Tuple(Mutable<i32>, String, #[shallow] Mutable<String>),
        Unit,
    }

impl Default for SomeEnum {
    fn default() -> Self {
        Self::Unit
    }
}

#[tokio::test]
async fn test_nested_observable() {
    let mut a = Arc::new(TestA::default());
    let change_count = Mutable::new(0);
    let change_count_cloned = change_count.clone();

    let a_cloned = a.clone();

    tokio::spawn(async move {
        let changes = a_cloned.changed();

        changes
            .for_each(|_| {
                println!("change detected");
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

    a.c.c.lock_mut().push_cloned(0.into());

    change_count.set(0);

    while change_count.get() < 20 {
        let v = a.c.c.lock_mut();
        v.get(0).unwrap().set(v.get(0).unwrap().get() + 1);

        yield_now().await;
    }

    change_count.set(0);
    a.enum_.set(SomeEnum::Struct { a: Default::default(), b: "".to_string(), shallow: Default::default() });

    while change_count.get() < 20 {
        {
            let v = a.enum_.lock_mut();
            match &*v {
                SomeEnum::Struct { a, .. } => {
                    a.set(a.get() + 1)
                }
                SomeEnum::Tuple(_, _, _) => {}
                SomeEnum::Unit => {}
            };
        }

        yield_now().await;
    }

    change_count.set(0);
    a.enum_.set(SomeEnum::Struct { a: Default::default(), b: "".to_string(), shallow: Default::default() });

    while change_count.get() < 20 {
        {
            let v = a.enum_.lock_mut();
            match &*v {
                SomeEnum::Struct { shallow, .. } => {
                    shallow.set(UnObservable {})
                }
                SomeEnum::Tuple(_, _, _) => {}
                SomeEnum::Unit => {}
            };
        }

        yield_now().await;
    }

    change_count.set(0);
    a.enum_.set(SomeEnum::Tuple ( Default::default(), "".to_string(), Default::default()));

    while change_count.get() < 20 {
        {
            let v = a.enum_.lock_mut();
            match &*v {
                SomeEnum::Struct {  .. } => {
                }
                SomeEnum::Tuple(a, _, _) => {
                    a.set(a.get() + 1)

                }
                SomeEnum::Unit => {}
            };
        }

        yield_now().await;
    }
}

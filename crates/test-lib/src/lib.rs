#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt;
    use futures_signals::signal::Mutable;
    use futures_signals::signal_vec::MutableVec;
    use futures_signals_observable::Observable;

    #[derive(Observable, Default)]
    struct TestB {
        a: Mutable<u32>,
        b: MutableVec<String>,
        c: MutableVec<Mutable<i32>>,
    }

    #[test]
    fn it_works() {}
}

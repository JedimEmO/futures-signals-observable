#[cfg(test)]
mod tests {
    use futures_signals::signal::Mutable;
    use futures_signals::signal_vec::MutableVec;
    use futures_signals_observable::Observable;
    use futures::StreamExt;
    use super::*;


    #[derive(Observable, Default)]
    struct TestB {
        a: Mutable<u32>,
        b: MutableVec<String>,
        c: MutableVec<Mutable<i32>>,
    }

    #[test]
    fn it_works() {
    }
}

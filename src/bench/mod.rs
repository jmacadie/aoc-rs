pub fn bench(f: fn()) -> took::Took {
    bench_inner(f)
}

fn bench_inner(f: fn()) -> took::Took {
    let _gag = gag::Gag::stdout().unwrap();
    let timer = took::Timer::new();
    f();
    timer.took()
    // gag dropped here
}

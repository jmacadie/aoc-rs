pub fn bench(f: fn()) -> std::time::Duration {
    let estimate = bench_inner(f);
    if estimate.as_secs() > 0 {
        return estimate;
    }

    // How many runs can we fit in 2 seconds
    let mut runs = std::time::Duration::from_secs(2).as_nanos() / estimate.as_nanos();
    runs = std::cmp::min(runs, 100); // ... but don't do more than 100, that would be silly

    // TODO: Ideally would return an average time and runtime range (after throwing out outliers)
    // as opposed to the single min time output currently
    //let out = (0..runs).map(|_| bench_inner(f)).collect::<Vec<_>>();

    (0..runs).map(|_| bench_inner(f)).min().unwrap()
}

fn bench_inner(f: fn()) -> std::time::Duration {
    let _gag = gag::Gag::stdout().unwrap();
    let timer = std::time::Instant::now();
    f();
    timer.elapsed()
    // gag dropped here
}

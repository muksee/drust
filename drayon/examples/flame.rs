fn main() {
    let guard = pprof::ProfilerGuard::new(100).unwrap();
    if let Ok(report) = guard
        .report()
        .build()
    {
        println!("report: {:?}", &report);
    };
}

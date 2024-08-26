run:
	 cargo test -- --nocapture

release:
	RUSTFLAGS="-C target-cpu=native" cargo build --release


perf:
	perf record -F 999 --call-graph dwarf -g target/release/interview

perf-report:
	perf report

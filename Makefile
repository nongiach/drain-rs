test:
	cargo run --release --example parser ./data/test_b.log

show_patterns:
	cargo run --release --example show_patterns ./data/test_b.log

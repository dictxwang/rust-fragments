# rust-fragments

# run test sample
1、at project root folder
cargo test -p learning --release -- --nocapture -- --test test1
cargo run -p about_async --bin about_arc_mutex

2、at project member folder, for example "learning"
cargo test --release -- --nocapture -- --test test1
cargo run --bin about_arc_mutex
use gamedig::{minetest_master_server, TimeoutSettings};

fn main() {
    println!(
        "{:#?}",
        minetest_master_server::query(TimeoutSettings::default())
    )
}

use gamedig::valve_master_server::{query, Filter, Region, SearchFilters};

fn main() {
    let search_filters = SearchFilters::new()
        .insert(Filter::RunsAppID(440))
        .insert(Filter::CanBeEmpty(false))
        .insert(Filter::CanBeFull(false))
        .insert(Filter::CanHavePassword(false))
        .insert(Filter::IsSecured(true))
        .insert(Filter::HasTags(&["minecraft"]));

    let ips = query(Region::Europe, Some(search_filters)).unwrap();
    println!("Servers: {:?} \n Amount: {}", ips, ips.len());
}

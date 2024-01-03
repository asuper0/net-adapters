pub mod adapter;

#[test]
fn get_adapters() {
    let adapters = adapter::get_adapters();
    assert_ne!(adapters.len(), 0);
}

use kourain_core::ServiceProvider;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[test]
fn singleton_returns_same_instance() {
    let mut provider = ServiceProvider::new();
    provider.add_singleton::<String>("kourain".to_string());

    let first = provider
        .get_service::<String>()
        .expect("singleton String should be available");
    let second = provider
        .get_service::<String>()
        .expect("singleton String should be available");

    assert_eq!(first.as_str(), "kourain");
    assert!(Arc::ptr_eq(&first, &second));
}

#[test]
fn scope_returns_new_instance_each_time() {
    let mut provider = ServiceProvider::new();
    let counter = Arc::new(AtomicUsize::new(0));
    let counter_for_factory = Arc::clone(&counter);

    provider.add_scope::<usize, _>(move || {
        counter_for_factory.fetch_add(1, Ordering::SeqCst) + 1
    });

    let first = provider
        .get_service::<usize>()
        .expect("scoped usize should be available");
    let second = provider
        .get_service::<usize>()
        .expect("scoped usize should be available");

    assert_eq!(*first, 1);
    assert_eq!(*second, 2);
    assert!(!Arc::ptr_eq(&first, &second));
}


use algorithm::LfuCache;
fn main() {
    let mut lru = LfuCache::new(3);
    lru.insert("hello", "algorithm");
    lru.insert("this", "lru");
    lru.set_reduce_count(100);
    assert!(lru.get_visit(&"hello") == Some(5));
    assert!(lru.get_visit(&"this") == Some(5));
    for _ in 0..98 {
        let _ = lru.get("this");
    }
    lru.insert("hello", "new");
    assert!(lru.get_visit(&"this") == Some(51));
    assert!(lru.get_visit(&"hello") == Some(3));
    let mut keys = lru.keys();
    assert!(keys.next()==Some(&"this"));
    assert!(keys.next()==Some(&"hello"));
    assert!(keys.next() == None);
}
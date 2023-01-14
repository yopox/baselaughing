use std::collections::HashSet;
use crate::from_u10;

#[test]
fn test_distinct() {
    let mut map = HashSet::new();

    for i in 0..1024 {
        map.insert(from_u10(i));
    }

    assert_eq!(map.len(), 1024);
}
extern crate service_utils;

use service_utils::islands::Territory;

#[test]
fn test_add_land() {
    let mut territory = Territory::new();
    assert_eq!(territory.add_land(0, 0), 1);
    assert_eq!(territory.add_land(2, 0), 2);
    assert_eq!(territory.add_land(1, 0), 1);
}

use snowflake::{SnowflakeGenerator, MAX_DATACENTER_ID, MAX_MACHINE_ID};

#[test]
fn ids_are_unique() {
    let gen = SnowflakeGenerator::new(0, 0);
    let ids: Vec<u64> = (0..1_000).map(|_| gen.next_id()).collect();
    let unique: std::collections::HashSet<_> = ids.iter().collect();
    assert_eq!(ids.len(), unique.len(), "duplicate IDs generated");
}

#[test]
fn ids_are_monotonically_increasing() {
    let gen = SnowflakeGenerator::new(0, 1);
    let ids: Vec<u64> = (0..100).map(|_| gen.next_id()).collect();
    for w in ids.windows(2) {
        assert!(w[1] > w[0], "ID order violated: {} >= {}", w[0], w[1]);
    }
}

#[test]
fn parse_round_trips() {
    let gen = SnowflakeGenerator::new(2, 5);
    let id = gen.next_id();
    let p = SnowflakeGenerator::parse(id);
    assert_eq!(p.datacenter_id, 2);
    assert_eq!(p.machine_id, 5);
    assert!(p.sequence < 4096);
}

#[test]
fn boundary_datacenter_and_machine() {
    let gen = SnowflakeGenerator::new(MAX_DATACENTER_ID, MAX_MACHINE_ID);
    let id = gen.next_id();
    let p = SnowflakeGenerator::parse(id);
    assert_eq!(p.datacenter_id, MAX_DATACENTER_ID);
    assert_eq!(p.machine_id, MAX_MACHINE_ID);
}

#[test]
#[should_panic]
fn out_of_range_datacenter_panics() {
    SnowflakeGenerator::new(MAX_DATACENTER_ID + 1, 0);
}

#[test]
#[should_panic]
fn out_of_range_machine_panics() {
    SnowflakeGenerator::new(0, MAX_MACHINE_ID + 1);
}

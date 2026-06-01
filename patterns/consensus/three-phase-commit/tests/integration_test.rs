use three_phase_commit::{Coordinator, Decision};

#[test]
fn commits_through_all_three_phases() {
    let coord = Coordinator::new(3, false);
    assert_eq!(coord.run(), Decision::Commit);
}

#[test]
fn aborts_when_coordinator_crashes_after_phase1() {
    let coord = Coordinator::new(3, true);
    assert_eq!(coord.run(), Decision::Abort);
}

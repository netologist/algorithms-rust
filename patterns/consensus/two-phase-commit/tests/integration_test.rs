use two_phase_commit::{Coordinator, Decision, Participant, ParticipantVote};

#[test]
fn commits_when_all_vote_yes() {
    let coord = Coordinator::new(vec![
        Participant::new(1, ParticipantVote::Yes),
        Participant::new(2, ParticipantVote::Yes),
        Participant::new(3, ParticipantVote::Yes),
    ]);
    assert_eq!(coord.run(), Decision::Commit);
}

#[test]
fn aborts_when_any_votes_no() {
    let coord = Coordinator::new(vec![
        Participant::new(1, ParticipantVote::Yes),
        Participant::new(2, ParticipantVote::No),
        Participant::new(3, ParticipantVote::Yes),
    ]);
    assert_eq!(coord.run(), Decision::Abort);
}

#[test]
fn aborts_when_all_vote_no() {
    let coord = Coordinator::new(vec![
        Participant::new(1, ParticipantVote::No),
        Participant::new(2, ParticipantVote::No),
    ]);
    assert_eq!(coord.run(), Decision::Abort);
}

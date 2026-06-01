use two_phase_commit::{Coordinator, Participant, ParticipantVote};

fn main() {
    println!("=== Two-Phase Commit Demo ===\n");

    println!("--- Scenario 1: All participants vote YES ---");
    let coord = Coordinator::new(vec![
        Participant::new(1, ParticipantVote::Yes),
        Participant::new(2, ParticipantVote::Yes),
        Participant::new(3, ParticipantVote::Yes),
    ]);
    let decision = coord.run();
    println!("  → Decision: {:?}\n", decision);

    println!("--- Scenario 2: Participant-2 votes NO ---");
    let coord = Coordinator::new(vec![
        Participant::new(1, ParticipantVote::Yes),
        Participant::new(2, ParticipantVote::No),
        Participant::new(3, ParticipantVote::Yes),
    ]);
    let decision = coord.run();
    println!("  → Decision: {:?}", decision);
}

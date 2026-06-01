use merkle_tree::MerkleTree;

#[test]
fn root_is_deterministic() {
    let data = ["hello", "world", "foo", "bar"];
    let t1 = MerkleTree::new(&data);
    let t2 = MerkleTree::new(&data);
    assert_eq!(t1.root(), t2.root());
}

#[test]
fn empty_tree_has_no_root() {
    let empty: &[&str] = &[];
    let t = MerkleTree::new(empty);
    assert_eq!(t.root(), None);
}

#[test]
fn single_element_tree() {
    let t = MerkleTree::new(&["only"]);
    assert!(t.root().is_some());
    assert_eq!(t.leaves().len(), 1);
}

#[test]
fn odd_number_of_leaves() {
    let t = MerkleTree::new(&["a", "b", "c"]);
    assert!(t.root().is_some());
    assert_eq!(t.leaves().len(), 3);
}

#[test]
fn modified_data_changes_root() {
    let original = MerkleTree::new(&["a", "b", "c", "d"]);
    let tampered = MerkleTree::new(&["a", "b", "TAMPERED", "d"]);
    assert_ne!(original.root(), tampered.root());
}

#[test]
fn verify_leaf_detects_tampering() {
    let t = MerkleTree::new(&["x", "y", "z"]);
    assert!(t.verify_leaf(0, "x"));
    assert!(!t.verify_leaf(0, "tampered"));
}

#[test]
fn matches_identical_datasets() {
    let data = ["p", "q", "r"];
    let t1 = MerkleTree::new(&data);
    let t2 = MerkleTree::new(&data);
    assert!(t1.matches(&t2));
}

#[test]
fn does_not_match_different_datasets() {
    let t1 = MerkleTree::new(&["a", "b"]);
    let t2 = MerkleTree::new(&["a", "c"]);
    assert!(!t1.matches(&t2));
}

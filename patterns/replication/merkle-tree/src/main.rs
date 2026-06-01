use merkle_tree::MerkleTree;

fn main() {
    let dataset_a = ["block-0", "block-1", "block-2", "block-3"];
    let dataset_b = ["block-0", "block-1", "MODIFIED", "block-3"];

    let tree_a = MerkleTree::new(&dataset_a);
    let tree_b = MerkleTree::new(&dataset_b);

    println!("=== Merkle Tree Demo ===\n");

    println!("Dataset A root: {}", tree_a.root().unwrap());
    println!("Dataset B root: {}", tree_b.root().unwrap());
    println!("Trees match  : {}", tree_a.matches(&tree_b));

    println!("\nLeaf hashes for Dataset A:");
    for (i, leaf) in tree_a.leaves().iter().enumerate() {
        println!("  leaf[{i}]: {}…", &leaf[..16]);
    }

    println!("\nTamper detection:");
    println!(
        "  leaf[2] unchanged in A? {}",
        tree_a.verify_leaf(2, "block-2")
    );
    println!(
        "  leaf[2] unchanged in B? {}",
        tree_b.verify_leaf(2, "block-2")
    );
}

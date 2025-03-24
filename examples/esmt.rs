use geosteiner::euclidean_steiner_tree;

fn main() {
    //this is the highlevel example from the geosteiner manual
    let terms = [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
    let tree = euclidean_steiner_tree(&terms);
    println!("found tree with length {}", tree.length);
    println!("steiner points: ");
    println!("{:.2?}", tree.steiner_points);
    println!("edges: ");
    println!("{:?}", tree.edges);
}
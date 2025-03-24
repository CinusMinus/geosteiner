use geosteiner::rectilinear_steiner_tree;

fn main() {
    let terms = [[0.0, 0.0], [2.0, 0.0], [1.0, 1.0], [1.5, -1.0]];
    let tree = rectilinear_steiner_tree(&terms);
    println!("found tree with length {}", tree.length);
    println!("steiner points: ");
    println!("{:.2?}", tree.steiner_points);
    println!("edges: ");
    println!("{:?}", tree.edges);
}
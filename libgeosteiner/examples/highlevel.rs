use libgeosteiner_sys::{Gst, EuclideanTree};

fn main() {
    //this is the highlevel example from the geosteiner manual
    let terms = [(0.0, 0.0), (1.0, 0.0), (0.0, 1.0), (1.0, 1.0)];
    let gst = Gst::new();
    let tree: EuclideanTree<(f64, f64)> = gst.esmt(terms.iter());
    println!("resulting tree with length {}", tree.length);
    println!("steiner points: ");
    tree.points.iter().skip(terms.len()).for_each(|pos| println!("{:?}", pos));
    println!("edges: ");
    tree.edges.iter().for_each(|(u,v)| println!("{} {}", u, v));
}
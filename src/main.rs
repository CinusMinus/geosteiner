use std::env::args;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use geosteiner::{EuclideanTree, Gst};
use itertools::Itertools;

fn read_from_file(path_buf: PathBuf) -> Vec<(f64, f64)> {
    let file = std::fs::File::open(path_buf).unwrap();
    let reader = BufReader::new(file);
    let mut points = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        let x = iter.next().unwrap().parse::<f64>().unwrap();
        let y = iter.next().unwrap().parse::<f64>().unwrap();
        points.push((x, y));
    }
    points
}

fn main() {
    let gst = Gst::new();
    let args: Vec<_> = args().collect();
    let points = read_from_file(PathBuf::from(&args[1]));
    let tree: EuclideanTree<(f64, f64)> = gst.esmt(&points);
    println!("steiner = [{}]", tree.points[points.len()..].iter().map(|(x, y)| format!("({x}, {y})")).join(", "));
    println!("edges = [{}]", tree.edges.iter().map(|(i, j)| format!("({i}, {j})")).join(", "));
    //println!("{} {}", tree.points.len(), tree.points.len() - points.len());
    //for point in tree.points {
    //    println!("{} {}", point.0, point.1);
    //}
    //println!("{}", tree.edges.len());
    //for edge in tree.edges {
    //    println!("{} {}", edge.0, edge.1);
    //}
}

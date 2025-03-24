#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::bindings::{gst_esmt, gst_open_geosteiner, gst_rsmt};
use std::os::raw::{c_double, c_int};
use std::ptr::null_mut;
use std::sync::Once;

#[allow(dead_code, deref_nullptr, clippy::all)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub struct SteinerTree {
    pub steiner_points: Vec<[f64; 2]>,
    pub edges: Vec<[usize; 2]>,
    pub length: f64,
}

static GST_INITIALIZED: Once = Once::new();
fn init_gst() {
    GST_INITIALIZED.call_once(|| unsafe {
        if gst_open_geosteiner() != 0 {
            panic!("could not open geosteiner");
        }
    });
}

pub fn euclidean_steiner_tree(points: &[[f64; 2]]) -> SteinerTree {
    let mut length: f64 = 0.0;
    let mut num_steiner = 0;
    let mut steiner_coords = vec![0.0; 2 * points.len()];
    let mut num_edges = 0;
    let mut edges = vec![0 as c_int; 2 * points.len() + 2];
    unsafe {
        init_gst();
        let ok = gst_esmt(
            points.len() as c_int,
            points.as_ptr() as *mut c_double,
            &mut length,
            &mut num_steiner,
            steiner_coords.as_mut_ptr(),
            &mut num_edges,
            edges.as_mut_ptr(),
            null_mut(),
            null_mut(),
        );
        if ok != 0 {
            panic!("geosteiner failed with error code {}", ok);
        }
    }
    steiner_coords.truncate(2 * num_steiner as usize);
    edges.truncate(2 * num_edges as usize);
    reconstruct_tree(length, steiner_coords, edges)
}

fn reconstruct_tree(
    length: f64,
    steiner_coords: Vec<f64>,
    edges: Vec<c_int>,
) -> SteinerTree {
    SteinerTree {
        steiner_points: steiner_coords.chunks(2).map(|c| [c[0], c[1]]).collect(),
        edges: edges
            .chunks(2)
            .map(|c| [c[0] as usize, c[1] as usize])
            .collect(),
        length,
    }
}

pub fn rectilinear_steiner_tree(points: &[[f64; 2]]) -> SteinerTree {
    let mut length: f64 = 0.0;
    let mut num_steiner = 0;
    let mut steiner_coords = vec![0.0; 2 * points.len()];
    let mut num_edges = 0;
    let mut edges = vec![0 as c_int; 2 * points.len()];
    unsafe {
        init_gst();
        let ok = gst_rsmt(
            points.len() as c_int,
            points.as_ptr() as *mut c_double,
            &mut length,
            &mut num_steiner,
            steiner_coords.as_mut_ptr(),
            &mut num_edges,
            edges.as_mut_ptr(),
            null_mut(),
            null_mut(),
        );
        if ok != 0 {
            panic!("geosteiner failed with error code {}", ok);
        }
    }
    steiner_coords.truncate(2 * num_steiner as usize);
    edges.truncate(2 * num_edges as usize);
    reconstruct_tree(length, steiner_coords, edges)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let terminals = [[0.0, 0.0], [0.0, 1.0]];
        let tree = euclidean_steiner_tree(&terminals);
        assert_eq!([[0.0f64, 0.0]; 0], *tree.steiner_points);
        assert_eq!(1.0, tree.length);
        assert_eq!([[0, 1]], tree.edges.as_slice());
    }

    #[test]
    fn square() {
        let terms = [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
        let tree = euclidean_steiner_tree(&terms);
        assert_eq!(2, tree.steiner_points.len());
        assert_eq!(5, tree.edges.len(), "unecpected edges: {:?}", tree.edges);
        assert!(2.7320 < tree.length, "2.7320 >= {}", tree.length);
        assert!(2.7321 > tree.length, "2.7321 <= {}", tree.length);
    }
}

#![doc = include_str!("../README.md")]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![warn(missing_docs)]

use crate::bindings::{gst_esmt, gst_open_geosteiner, gst_rsmt};
use std::os::raw::{c_double, c_int};
use std::ptr::null_mut;
use std::sync::Once;

#[allow(dead_code, deref_nullptr, clippy::all)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

/// A Steiner tree constructed on a separate set of terminals as returned by the Steiner tree methods.
///
/// Contains the steiner points and edges as well the length of the tree.
/// Terminal positions are stored externally.
pub struct SteinerTree {
    /// Steiner point positions
    pub steiner_points: Vec<[f64; 2]>,
    /// Edges as pairs of (start, end)
    ///
    /// start and end indices in the range 0..terminals.len() refer to terminals.
    /// The remaining indices refer to steiner points offset by the number of terminals.
    pub edges: Vec<[usize; 2]>,
    /// Total edge length of the Steiner tree given in the respective metric.
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

/// Construct a Euclidean minimum Steiner tree (ESMT) on `points`.
pub fn euclidean_steiner_tree(points: &[[f64; 2]]) -> SteinerTree {
    let mut length: f64 = 0.0;
    let mut num_steiner = 0;
    let mut steiner_coords = vec![0.0; 2 * points.len()];
    let mut num_edges = 0;
    let mut edges = vec![0 as c_int; 4 * points.len()];
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

/// Construct a rectilinear minimum Steiner tree (RSMT) on the given terminals.
pub fn rectilinear_steiner_tree(points: &[[f64; 2]]) -> SteinerTree {
    let mut length: f64 = 0.0;
    let mut num_steiner = 0;
    let mut steiner_coords = vec![0.0; 2 * points.len()];
    let mut num_edges = 0;
    let mut edges = vec![0 as c_int; 4 * points.len()];
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
    fn line() {
        let terminals = [[0.0, 0.0], [0.0, 1.0]];
        let esmt = euclidean_steiner_tree(&terminals);
        assert_eq!([[0.0f64, 0.0]; 0], *esmt.steiner_points);
        assert_eq!(1.0, esmt.length);
        assert_eq!([[0, 1]], esmt.edges.as_slice());
        let rsmt = rectilinear_steiner_tree(&terminals);
        assert!(esmt.steiner_points.is_empty());
        assert_eq!(esmt.length, 1.0);
        assert_eq!(esmt.edges, rsmt.edges);
    }

    #[test]
    fn square() {
        let terms = [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
        let esmt = euclidean_steiner_tree(&terms);
        assert_eq!(2, esmt.steiner_points.len());
        assert_eq!(5, esmt.edges.len(), "unecpected edges: {:?}", esmt.edges);
        assert!(2.7320 < esmt.length, "2.7320 >= {}", esmt.length);
        assert!(2.7321 > esmt.length, "2.7321 <= {}", esmt.length);

        let rsmt = rectilinear_steiner_tree(&terms);
        assert_eq!(3.0, rsmt.length);
        assert!(rsmt.steiner_points.is_empty());
        assert_eq!(3, rsmt.edges.len());
    }

    #[test]
    fn grid_4x4() {
        let grid = (0..4).flat_map(|x| (0..4).map(move |y| [x as f64, y as f64])).collect::<Vec<_>>();
        let esmt = euclidean_steiner_tree(&grid);
        assert_eq!(10, esmt.steiner_points.len());
        assert_eq!(25, esmt.edges.len());
        assert!(5.0 * 2.7320 < esmt.length, "5 * 2.7320 >= {}", esmt.length);
        assert!(5.0 * 2.7321 > esmt.length, "5 * 2.7321 <= {}", esmt.length);
        let rsmt = rectilinear_steiner_tree(&grid);
        assert_eq!(15.0, rsmt.length);
        assert!(rsmt.steiner_points.is_empty());
    }
}

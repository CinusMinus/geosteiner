#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::sync::Once;
use crate::bindings::{gst_open_geosteiner, gst_esmt};
use std::os::raw::{c_double, c_int};
use std::ptr::null_mut;

#[allow(dead_code,deref_nullptr,clippy::all)]
mod bindings;

pub trait Positioned {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
}

impl Positioned for (f64, f64) {
    fn x(&self) -> f64 {
        self.0
    }

    fn y(&self) -> f64 {
        self.1
    }
}

impl Positioned for &(f64, f64) {
    fn x(&self) -> f64 {
        self.0
    }

    fn y(&self) -> f64 {
        self.1
    }
}

impl Positioned for [f64; 2] {
    fn x(&self) -> f64 {
        self[0]
    }

    fn y(&self) -> f64 {
        self[1]
    }
}

impl Positioned for &[f64; 2] {
    fn x(&self) -> f64 {
        self[0]
    }

    fn y(&self) -> f64 {
        self[1]
    }
}

pub trait FromSteiner : Positioned {
    fn from_terminal(x: f64, y: f64) -> Self;
    fn from_steiner(x: f64, y: f64) -> Self;
}

impl FromSteiner for (f64, f64) {
    fn from_terminal(x: f64, y: f64) -> Self {
        (x,y)
    }

    fn from_steiner(x: f64, y: f64) -> Self {
        (x,y)
    }
}

impl FromSteiner for [f64; 2] {
    fn from_terminal(x: f64, y: f64) -> Self {
        [x, y]
    }

    fn from_steiner(x: f64, y: f64) -> Self {
        [x, y]
    }
}
static GST_INITIALIZED: Once = Once::new();

#[derive(Clone, Debug, PartialEq)]
pub struct EuclideanTree<T: Positioned> {
    pub points: Vec<T>,
    pub edges: Vec<(usize, usize)>,
    pub length: f64,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Gst(());

impl Gst {
    pub fn new() -> Self {
        GST_INITIALIZED.call_once(|| unsafe {
            if gst_open_geosteiner() != 0 {
                panic!("could not open geosteiner");
            }
        });
        Self(())
    }

    #[allow(clippy::unused_self)]
    pub fn esmt<T: FromSteiner, U: Positioned, It: IntoIterator<Item = U>>(self, terminals: It) -> EuclideanTree<T>{
        let iter = terminals.into_iter();
        let mut terms: Vec<c_double> = Vec::with_capacity(iter.size_hint().0 * 2);
        for pos in iter {
            terms.push(pos.x() as c_double);
            terms.push(pos.y() as c_double);
        }
        let mut length: c_double = 0.0;
        let term_count = (terms.len() / 2) as c_int;
        let mut steiner_count: c_int = 0;
        let mut steiner_coords: Vec<c_double> = vec![0.0; (term_count * 2) as usize];

        let mut steiner_edge_count: c_int = 0;
        let mut steiner_edges = vec![0; (term_count * 2 * 2) as usize];
        unsafe {
            gst_esmt(
                term_count as c_int, terms.as_ptr() as *mut c_double,
                &mut length,
                &mut steiner_count,
                steiner_coords.as_mut_ptr(),
                &mut steiner_edge_count, steiner_edges.as_mut_ptr(),
                null_mut(), null_mut()
            );
        }
        steiner_coords.resize_with(2 * (steiner_count as usize), || panic!("should have been large enough"));
        steiner_edges.resize_with(2 * (steiner_edge_count as usize), || panic!("should have been large enough"));
        fn map_chunks<T, U: Copy, F: Fn(U, U) -> T>(f: F) -> impl Fn(&[U]) -> T {
            move |w| if let [x,y] = *w {
                f(x, y)
            } else {
                panic!("expected even number of coordinates")
            }
        }
        let mut points = Vec::with_capacity(term_count as usize + steiner_count as usize);
        points.extend(terms.chunks(2).map(map_chunks(T::from_terminal)));
        points.extend(steiner_coords.chunks(2).map(map_chunks(T::from_steiner)));
        let edges: Vec<_> = steiner_edges.chunks(2).map(map_chunks(|x,y| (x as usize, y as usize))).collect();
        EuclideanTree { points, edges, length: length as f64 }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let terminals = [(0.0, 0.0), (0.0, 1.0)];
        let tree: EuclideanTree<(f64, f64)> = Gst::new().esmt(terminals.iter());
        assert_eq!(&terminals, tree.points.as_slice());
        assert_eq!(1.0, tree.length);
        assert_eq!([(0,1)], tree.edges.as_slice());

    }

    #[test]
    fn square() {
        let terms = [(0.0, 0.0), (1.0, 0.0), (0.0, 1.0), (1.0, 1.0)];
        let tree: EuclideanTree<(f64, f64)> = Gst::new().esmt(&terms);
        assert_eq!(6, tree.points.len());
        assert_eq!(5, tree.edges.len());
        for (i, &t) in terms.iter().enumerate() {
            assert_eq!(t, tree.points[i]);
        }
        assert!(2.7320 < tree.length, "2.7320 >= {}", tree.length);
        assert!(2.7321 > tree.length, "2.7321 <= {}", tree.length);
    }
}

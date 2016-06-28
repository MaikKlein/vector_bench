#![feature(test)]
extern crate num;
extern crate rand;
extern crate test;
use self::test::{Bencher, black_box};

use num::Float;
use rand::distributions::{IndependentSample, Range};
use rand::distributions::range::SampleRange;

#[derive(Copy, Clone)]
pub struct Vector<T>
    where T: Float
{
    data: [T; 4],
}

impl<T> Vector<T>
    where T: Float
{
    pub fn dot_ref(&self, other: &Self) -> T {
        Iterator::zip(self.data.iter(), other.data.iter())
            .fold(T::zero(), |acc, (a, b)| acc + *a * *b)
    }

    pub fn dot_copy(self, other: Self) -> T {
        Iterator::zip(self.data.iter(), other.data.iter())
            .fold(T::zero(), |acc, (a, b)| acc + *a * *b)
    }
}

pub fn create_random_vectors<T>(size: usize) -> Vec<Vector<T>>
    where T: Float + SampleRange
{
    use num::NumCast;
    let mut vec = Vec::with_capacity(size);
    let min: T = NumCast::from(-100.0).unwrap();
    let max: T = NumCast::from(100.0).unwrap();
    let between = Range::new(min, max);
    let mut rng = rand::thread_rng();

    for _ in 0..size {
        let a = between.ind_sample(&mut rng);
        let b = between.ind_sample(&mut rng);
        let c = between.ind_sample(&mut rng);
        let d = between.ind_sample(&mut rng);
        vec.push(Vector { data: [a, b, c, d] });
    }
    vec
}

pub fn calc_dots_ref<T: Float>(left: &Vec<Vector<T>>, right: &Vec<Vector<T>>) -> T {
    Iterator::zip(left.iter(), right.iter()).fold(T::zero(), |acc, (a, b)| acc + a.dot_ref(b))
}

pub fn calc_dots_copy<T: Float>(left: &Vec<Vector<T>>, right: &Vec<Vector<T>>) -> T {
    Iterator::zip(left.iter(), right.iter()).fold(T::zero(), |acc, (a, b)| acc + a.dot_copy(*b))
}

const N: usize = 1000000;
#[bench]
fn bref_f32(b: &mut Bencher) {
    let v = create_random_vectors::<f32>(N);
    let v1 = create_random_vectors::<f32>(N);
    b.iter(|| calc_dots_ref(&v, &v1));
}

#[bench]
fn copy_f32(b: &mut Bencher) {
    let v = create_random_vectors::<f32>(N);
    let v1 = create_random_vectors::<f32>(N);
    b.iter(|| calc_dots_copy(&v, &v1));
}

#[bench]
fn bref_f64(b: &mut Bencher) {
    let v = create_random_vectors::<f64>(N);
    let v1 = create_random_vectors::<f64>(N);
    b.iter(|| calc_dots_ref(&v, &v1));
}

#[bench]
fn copy_f64(b: &mut Bencher) {
    let v = create_random_vectors::<f64>(N);
    let v1 = create_random_vectors::<f64>(N);
    b.iter(|| calc_dots_copy(&v, &v1));
}

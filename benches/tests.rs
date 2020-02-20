#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use sequence_perf::*;
    use itertools::Itertools;
    use test::{Bencher, black_box};

    fn all3()  -> Vec<Vec<u32>> {
        let x = vec![1, 2, 3, 4];
        let y = vec![10, 20, 30, 40];
        let z = vec![100, 200, 300, 400];
        vec![x, y, z]
    } 

    fn all4()  -> Vec<Vec<u32>> {
        let x = vec![1, 2, 3, 4];
        let y = vec![10, 20, 30, 40];
        let z = vec![100, 200, 300, 400];
        let w = vec![1000, 2000, 3000, 4000];
        vec![x, y, z, w]
    }

    fn all()  -> Vec<Vec<u32>> {
        let x = vec![1, 2, 3, 4];
        let y = vec![10, 20, 30, 40];
        let z = vec![100, 200, 300, 400];
        let w = vec![1000, 2000, 3000, 4000];
        let a = vec![10000, 20000, 30000, 40000];
        let b = vec![10000, 20000, 30000, 40000];
        let c = vec![10000, 20000, 30000, 40000];
        let d = vec![10000, 20000, 30000, 40000];
        let e = vec![10000, 20000, 30000, 40000];
        let f = vec![10000, 20000, 30000, 40000];
        vec![x, y, z, w, a, b,c ,d, e, f]
    } 

    #[bench]
    fn sequence_test(b: &mut Bencher) {
        let all = all();
        b.iter(|| {
            sequence(&all)
        })
    }

    #[bench]
    fn sequence2_test(b: &mut Bencher) {
        let all = all();
        b.iter(|| {
            sequence2(&all)
        })
    }

    #[bench]
    fn sequence_idiomatic_test(b: &mut Bencher) {
        let all = all();
        b.iter(|| {
            sequence_idiomatic(&all)
        })
    }

    #[bench]
    fn multi_cartesian_product_test(b: &mut Bencher) {
        let all = all();
        b.iter(|| {
            all.iter().multi_cartesian_product().collect::<Vec<_>>()
        })
    }

    #[bench]
    fn sequence_by_row_test(b: &mut Bencher) {
        let all = all();
        b.iter(|| {
            sequence_by_row(&all)
        })
    }

    #[bench]
    fn sequence_by_column_test(b: &mut Bencher) {
        let all = all();
        b.iter(|| {
            sequence_by_column(&all)
        })
    }

    #[bench]
    fn sequence_by_rc_test(b: &mut Bencher) {
        let all = all();
        b.iter(|| {
            sequence_by_rc(&all)
        })
    }

    #[bench]
    fn sequence_by_row_reyon_test(b: &mut Bencher) {
        let all = all();
        b.iter(|| {
            sequence_by_row_rayon(&all)
        })
    }

    
}
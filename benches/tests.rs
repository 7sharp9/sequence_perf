#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use sequence_perf::{sequence_idiomatic, sequence, sequence2, sequence_by_row, sequence_by_column, sequence_by_rc};
    use itertools::Itertools;
    use test::{Bencher, black_box};

    fn all()  -> Vec<Vec<u32>> {
        let x = vec![1, 2, 3, 4];
        let y = vec![10, 20, 30, 40];
        let z = vec![100, 200, 300, 400];
        vec![x, y, z]
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
}
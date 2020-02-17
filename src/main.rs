use itertools::Itertools;
use sequence_perf::{sequence, sequence2, sequence_idiomatic};

fn main() {
    let x = vec![1, 2, 3];
    let y = vec![10, 20, 30];
    let z = vec![100, 200, 300];
    let all = vec![x, y, z];

    let r1 = all.iter().multi_cartesian_product().collect::<Vec<_>>();
    let r2 = sequence(&all);
    let r3 = sequence2(&all);
    let r4 = sequence_idiomatic(&all);

    println!("{:?}\n{:?}\n{:?}\n{:?}", r1, r2, r3, r4)
}

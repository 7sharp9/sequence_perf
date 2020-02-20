pub fn sequence_idiomatic<T>(v: &[Vec<T>]) -> Vec<Vec<&T>> {
    if let Some((item, items)) = v.split_first() {
        let state = sequence_idiomatic(items);
        let mut result = Vec::with_capacity(item.len() * state.len());
        for x in item {
            for xs in &state {
                let mut v = Vec::with_capacity(xs.len() + 1);
                v.push(x);
                v.extend(xs);
                result.push(v)
            }
        }
        result
    } else {
        vec![vec![]]
    }
}

pub fn sequence2<T>(v: &[Vec<T>]) -> Vec<Vec<&T>> {
    v.iter().rfold(vec![vec![]], |state, item| {
        item.iter()
            .flat_map(|x| {
                state.iter().map(move |xs| {
                    let mut v = vec![x];
                    v.extend(xs);
                    v
                })
            })
            .collect()
    })
}

pub fn sequence<T>(v: &[Vec<T>]) -> Vec<Vec<&T>> {
    v.iter().rfold(vec![vec![]], |state, item| {
        let mut result = vec![];
        for x in item {
            for xs in &state {
                let mut v = vec![x];
                v.extend(xs);
                result.push(v)
            }
        }
        result
    })
}

#[inline]
fn incr_counter<T>(counter: &mut [usize], v: &[Vec<T>]) {
    for (max, cntr) in v.iter().map(|vi| vi.len()).zip(counter).rev() {
        *cntr += 1;
        if *cntr == max {
            *cntr = 0;
        } else {
            break;
        }
    }
}

pub fn sequence_by_row<T>(v: &[Vec<T>]) -> Vec<Vec<&T>> {
    let count: usize = v.iter().map(|v| v.len()).product();

    let mut res = Vec::with_capacity(count);
    for _ in 0..count {
        res.push(Vec::with_capacity(v.len()));
    }

    let mut counter = vec![0; v.len()];
    for vec in &mut res {
        for (vi, idx) in v.iter().zip(&counter) {
            vec.push(&vi[*idx]);
        }
        incr_counter(&mut counter, v);
    }

    res
}

pub fn sequence_by_column<T>(v: &[Vec<T>]) -> Vec<Vec<&T>> {
    let count: usize = v.iter().map(|v| v.len()).product();

    let mut res = Vec::with_capacity(count);
    for _ in 0..count {
        res.push(Vec::with_capacity(v.len()));
    }

    let mut per_row = count;
    for vi in v {
        per_row /= v.len();
        let mut vidx = 0;
        let mut row_cnt = 0;

        for vec in &mut res {
            vec.push(&vi[vidx]);
            row_cnt += 1;
            if row_cnt == per_row {
                row_cnt = 0;
                vidx += 1;
                if vidx == vi.len() {
                    vidx = 0;
                }
            }
        }
    }

    res
}

use std::rc::Rc;
pub enum RcChain<'a, T> {
    Cons {
        value: &'a T,
        tail: Rc<RcChain<'a, T>>,
    },
    Nil,
}

fn folder<'a, T>(state: Vec<Rc<RcChain<'a, T>>>, item: &'a Vec<T>) -> Vec<Rc<RcChain<'a, T>>> {
    let mut res = Vec::with_capacity(item.len() * state.len());
    for x in item {
        for xs in &state {
            res.push(Rc::new(RcChain::Cons {
                value: x,
                tail: Rc::clone(xs),
            }));
        }
    }
    res
}

pub fn sequence_by_rc<T>(v: &[Vec<T>]) -> Vec<Rc<RcChain<T>>> {
    v.iter().rfold(vec![Rc::new(RcChain::Nil)], folder)
}

#[inline]
fn set_counter_to<T>(counter: &mut [usize], v: &[Vec<T>], val: usize) {
    let mut val = val;
    for (max, cntr) in v.iter().map(|vi| vi.len()).zip(counter).rev() {
        *cntr = val % max;
        val = val / max;
    }
}

fn sequence_by_row_task<'a, 'b, T>(v: &'a [Vec<T>], out: &'b mut [Vec<&'a T>], off: usize) {
    let mut counter = vec![0; v.len()];
    set_counter_to(&mut counter, v, off);
    for vec in out {
        for (vi, idx) in v.iter().zip(&counter) {
            vec.push(&vi[*idx]);
        }
        incr_counter(&mut counter, v);
    }
}

fn sequence_by_row_join<'a, 'b, T: Send + Sync>(v: &'a [Vec<T>], out: &'b mut [Vec<&'a T>], off: usize, num: usize) {
    if num == 1 {
        return sequence_by_row_task(v, out, off);
    }
    let left_len = out.len() / 2;
    let right_len = out.len() - left_len;
    let (left, right) = out.split_at_mut(left_len);
    rayon::join(
        || {
            sequence_by_row_join(v, left, off, left_len);
        },
        || {
            sequence_by_row_join(v, right, off + left_len, right_len);
        },
    );
}

pub fn sequence_by_row_rayon<T: Send + Sync>(v: &[Vec<T>]) -> Vec<Vec<&T>> {
    let count: usize = v.iter().map(|v| v.len()).product();

    let mut res = Vec::with_capacity(count);
    for _ in 0..count {
        res.push(Vec::with_capacity(v.len()));
    }

    sequence_by_row_join(v, &mut res, 0, rayon::current_num_threads());

    res
}
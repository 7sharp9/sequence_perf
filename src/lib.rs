pub fn sequence_idiomatic<T>(v: &[Vec<T>]) -> Vec<Vec<&T>> {
    if let Some((item, items)) = v.split_first() {
        let state = sequence_idiomatic(items);
        let mut result = vec![];
        for x in item {
            for xs in &state {
                let mut v = vec![x];
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
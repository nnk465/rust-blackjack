use std::collections::HashMap;

// au top-level
lazy_static! {
    static ref VALID_COMBOS: Vec<(usize,usize,usize)> = {
        let mut v = Vec::new();
        for i in 0..10 {
            for j in i..10 {
                for k in j..10 {
                    if i + j + k <= 21 {
                        v.push((i,j,k));
                    }
                }
            }
        }
        v
    };
    static ref COMBO_TO_INDEX: HashMap<(usize,usize,usize), usize> = {
        VALID_COMBOS
            .iter()
            .cloned()
            .enumerate()
            .map(|(idx, triple)| (triple, idx))
            .collect()
    };
}

fn combo_index(a: usize, b: usize, c: usize) -> usize {
    let mut v = [a,b,c];
    v.sort_unstable();
    COMBO_TO_INDEX[&(v[0],v[1],v[2])]
}

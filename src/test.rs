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

/// Index unique de la combinaison non-ordonnée (a,b,c),
/// parmi celles dont a+b+c ≤ 21.
fn combo_index(a: usize, b: usize, c: usize) -> usize {
    let mut v = [a, b, c];
    v.sort_unstable();
    let (x, y, z) = (v[0], v[1], v[2]);
    let mut idx = 0;

    // 1) pour tout i0 < x
    for i0 in 0..x {
        for j0 in i0..10 {
            for k0 in j0..10 {
                if i0 + j0 + k0 <= 21 {
                    idx += 1;
                }
            }
        }
    }

    // 2) pour i0 == x, j0 < y
    for j0 in x..y {
        for k0 in j0..10 {
            if x + j0 + k0 <= 21 {
                idx += 1;
            }
        }
    }

    // 3) pour i0 == x, j0 == y, k0 < z
    for k0 in y..z {
        if x + y + k0 <= 21 {
            idx += 1;
        }
    }

    idx
}


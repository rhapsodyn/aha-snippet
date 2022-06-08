use rand::{thread_rng, Rng};
use std::{collections::LinkedList, time::Instant};

const MAX_LEVEL: usize = 3;

struct Node {
    data: u32,                     // Copy for ease
    next_level_idx: Option<usize>, // last level will be all None
    prev_level_idx: Option<usize>, // first level will be all None
}

///
/// from top-left to bottom-right
/// last level contains all nodes
///
struct SkipList {
    heads: [Vec<Node>; MAX_LEVEL],
}

impl SkipList {
    fn len(&self) -> usize {
        self.heads[MAX_LEVEL - 1].len()
    }

    fn insert(&mut self, data: u32) -> bool {
        let mut updated = [0_usize; MAX_LEVEL];
        for l in 0..MAX_LEVEL {
            for i in 0..self.heads[l].len() {
                if self.heads[l][i].data == data {
                    return false;
                }

                if data < self.heads[l][i].data {
                    updated[l] = i;
                    break;
                } else if i == self.heads[l].len() - 1 {
                    updated[l] = i + 1;
                }
            }
        }

        let mut rlevel = MAX_LEVEL - 1;
        if updated[0] == 0 {
            // first column has to span all rows
            rlevel = 0;
        } else {
            while rlevel > 0 {
                match thread_rng().gen_bool(0.3) {
                    true => rlevel -= 1,
                    false => break,
                }
            }
        }

        for l in (rlevel..MAX_LEVEL).rev() {
            let i = updated[l];

            let mut next_level_idx = None;
            let mut prev_level_idx = None;

            if l > rlevel {
                prev_level_idx = Some(updated[l - 1]);
            }
            if l < MAX_LEVEL - 1 {
                next_level_idx = Some(updated[l + 1]);
            }

            self.heads[l].insert(
                i,
                Node {
                    data,
                    next_level_idx,
                    prev_level_idx,
                },
            );

            // update indexes of right part
            for right in (i + 1)..self.heads[l].len() {
                if let Some(p) = self.heads[l][right].prev_level_idx {
                    let old_n = self.heads[l - 1][p].next_level_idx.unwrap();
                    self.heads[l - 1][p].next_level_idx = Some(old_n + 1);
                }

                if let Some(n) = self.heads[l][right].next_level_idx {
                    let old_p = self.heads[l + 1][n].prev_level_idx.unwrap();
                    self.heads[l + 1][n].prev_level_idx = Some(old_p + 1);
                }
            }
        }

        true
    }

    fn search(&self, data: &u32) -> bool {
        if self.len() == 0 {
            return false;
        }

        let mut i = 0;
        let mut l = 0;
        loop {
            // reach the right end
            if i >= self.len() {
                break;
            }
            // reach the bottom end
            if l >= MAX_LEVEL {
                break;
            }

            let curr = self.heads[l][i].data;
            if curr == *data {
                return true;
            }

            if curr < *data {
                if i >= self.heads[l].len() - 1 {
                    // go down
                    l = l + 1;
                } else {
                    // go right
                    i = i + 1;
                }
            } else {
                // go down
                l = l + 1;
            }
        }

        false
    }

    // #[allow(dead_code)]
    // fn delete(&mut self, node: &Node) -> bool {
    //     todo!()
    // }

    fn new() -> Self {
        SkipList {
            heads: [vec![], vec![], vec![]],
        }
    }
}

const SAMPLE_COUNT: usize = 10_000;

fn shuffle(vec: &mut [u32]) {
    let mut rng = thread_rng();
    for n in 1..vec.len() {
        let idx = rng.gen_range(0..n);
        let tmp = vec[idx];
        vec[idx] = vec[n];
        vec[n] = tmp;
    }
}

/**
 * Vec would be fastest as expected
 */
fn perf() {
    let mut samples = [0u32; SAMPLE_COUNT];
    let mut rng = thread_rng();
    let max = SAMPLE_COUNT as u32;
    for i in 0..SAMPLE_COUNT {
        samples[i] = rng.gen_range(0..max);
    }
    shuffle(&mut samples);

    let mut sl = SkipList::new();
    let mut v = Vec::with_capacity(SAMPLE_COUNT);
    let mut ll = LinkedList::new();
    for s in samples {
        sl.insert(s);
        v.push(s);
        ll.push_back(s);
    }
    shuffle(&mut samples);

    let mut now = Instant::now();
    for s in samples {
        assert!(v.contains(&s));
    }
    println!("search in vec takes: {}ms", now.elapsed().as_millis());

    now = Instant::now();
    for s in samples {
        assert!(sl.search(&s));
    }
    println!("search in skiplist takes: {}ms", now.elapsed().as_millis());

    now = Instant::now();
    for s in samples {
        assert!(ll.contains(&s));
    }
    println!("search in linklist takes: {}ms", now.elapsed().as_millis());
}

#[test]
fn debug_structural_correct() {
    let mut sl = SkipList::new();
    sl.insert(8);
    sl.insert(2);
    sl.insert(9);
    sl.insert(6);
    sl.insert(1);
    sl.insert(4);
    sl.insert(5);
    sl.insert(7);
    sl.insert(3);
    sl.insert(10);

    assert_eq!(sl.len(), 5)
}

fn main() {
    perf();
}

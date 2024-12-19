use anyhow::*;

pub fn solve(input: &str) -> Result<(u32, u64)> {
    let split = memchr::memmem::find(input.as_bytes(), b"\n\n").context("No separator found")?;
    let patterns = &input[..split];
    let designs = &input[split+2..];
    
    let mut trie = Trie::new();
    for pattern in patterns.split(", ") {
        trie.insert(pattern.as_bytes());
    }

    let mut p1 = 0;
    let mut p2 = 0;

    for design in designs.lines() {
        let score = trie.count(design.as_bytes());
        if score > 0 {
            p1 += 1;
        }
        p2 += score;

    }
    Ok((p1, p2))
}


#[derive(Clone)]
struct Trie {
    here: bool,
    children: Option<Vec<Trie>>
}

impl Trie {
    fn new() -> Self {
        Self { here: false, children: None }
    }

    fn insert(&mut self, prefix: &[u8]) {
        if prefix.is_empty() {
            self.here = true;
        } else {
            if self.children.is_none() {
                self.children = Some(vec![Trie::new(); 26])
            }
            if let Some(children) = &mut self.children {
                children[(prefix[0] - b'a') as usize].insert(&prefix[1..]);
            }
        }
    }

    fn count(&self, design: &[u8]) -> u64 {
        let mut cache = vec![None; design.len()];
        self.count_aux(self, design, &mut cache, 0, true)
    }

    fn count_aux(&self, node: &Trie, design: &[u8], cache: &mut [Option<u64>], index: usize, start: bool) -> u64 {
        if index == design.len() {
            node.here as u64
        } else if start {
            if let Some(v) = cache[index] {
                v
            } else if let Some(children) = &self.children {
                let v = self.count_aux(&children[(design[index] - b'a') as usize], design, cache, index + 1, false);
                cache[index] = Some(v);
                v
            } else {
                0
            }
        } else {
            let v1 =
                if let Some(children) = &node.children {
                   self.count_aux(&children[(design[index] - b'a') as usize], design, cache, index + 1, false)
                } else {
                    0
                };
            let v2 =
                if node.here {
                    self.count_aux(self, design, cache, index, true)
                } else {
                    0
                };
            v1 + v2
        }
    }

}
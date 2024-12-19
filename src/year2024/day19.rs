use anyhow::*;

pub fn solve(input: &str) -> Result<(u32, u64)> {
    let split = memchr::memmem::find(input.as_bytes(), b"\n\n").context("No separator found")?;
    let patterns = &input[..split];
    let designs = &input[split+2..];
    
    let mut trie = Trie::with_capacity(3000);
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

 
struct TrieNode {
    here: bool,
    children: Option<[usize; 5]>
}

struct Trie {
    nodes: Vec<TrieNode>,
}

impl Trie {
    fn with_capacity(n: usize) -> Self {
        let mut nodes = Vec::with_capacity(n);
        nodes.push(TrieNode { here: false, children: None });
        Self { nodes }
    }

    fn insert(&mut self, word: &[u8]) {
        let mut node_index =  0;
        for &c in word {
            match self.nodes[node_index].children {
                None => {
                    let size = self.nodes.len();
                    for _ in 0..5 {
                        self.nodes.push(TrieNode { here: false, children: None });
                    }
                    self.nodes[node_index].children = Some(std::array::from_fn(|i| size + i));
                    node_index = size + TABLE[c as usize];
                },
                Some(children) => {
                    node_index = children[TABLE[c as usize]];
                }
            }
        }
        self.nodes[node_index].here = true;
    }

    fn count(&self, design: &[u8]) -> u64 {
        let mut cache = vec![0; design.len()+1];
        cache[0] = 1;
        for i in 0..design.len() {
            if cache[i] != 0 {
                let mut node_index = 0;
                let mut index = i;
                loop {
                    if self.nodes[node_index].here {
                        cache[index] += cache[i];
                    }
                    if index >= design.len() {
                        break;
                    }
                    if let Some(children) = &self.nodes[node_index].children {
                        node_index = children[TABLE[design[index] as usize]];
                        index += 1;
                    } else {
                        break;
                    }
                }
            }
        }
        cache[design.len()]
    }
}

const fn mk_table() -> [usize; 128] {
    let mut table = [0; 128];
    table[b'w' as usize] = 1;
    table[b'u' as usize] = 2;
    table[b'b' as usize] = 3;
    table[b'r' as usize] = 4;
    table
}

const TABLE: [usize; 128] = mk_table();
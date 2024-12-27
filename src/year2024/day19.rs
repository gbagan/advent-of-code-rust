// dynamic programming

pub fn solve(input: &str) -> (u32, u64) {
    let split = memchr::memmem::find(input.as_bytes(), b"\n\n").unwrap();
    let patterns = &input[..split];
    let designs = &input[split+2..];
    
    let mut trie = Trie::with_capacity(1000);
    for pattern in patterns.split(", ") {
        trie.insert(pattern.as_bytes());
    }

    let mut p1 = 0;
    let mut p2 = 0;

    let mut table = Vec::with_capacity(75);

    for design in designs.lines() { 
        let score = trie.count(design.as_bytes(), &mut table);
        p1 += (score > 0) as u32;
        p2 += score;
    }
    (p1, p2)
}

struct TrieNode {
    here: bool,
    children: [u16; 5]
}

struct Trie {
    nodes: Vec<TrieNode>,
}

impl Trie {
    fn with_capacity(n: usize) -> Self {
        let mut nodes = Vec::with_capacity(n);
        nodes.push(TrieNode { here: false, children: [0; 5] });
        Self { nodes }
    }

    fn insert(&mut self, word: &[u8]) {
        let mut node_index =  0;
        for &c in word {
            let idx = COLOR_INDEX[c as usize];
            match self.nodes[node_index].children[idx] { 
                0 => {
                    let size = self.nodes.len();
                    self.nodes.push(TrieNode { here: false, children: [0; 5] });
                    self.nodes[node_index].children[idx] = size as u16;
                    node_index = size;
                },
                n => node_index = n as usize
            }
        }
        self.nodes[node_index].here = true;
    }

    fn count(&self, design: &[u8], table: &mut Vec<u64>) -> u64 {
        table.clear();
        table.resize(design.len() + 1, 0);
        table[0] = 1;
        for i in 0..design.len() {
            if table[i] != 0 {
                let mut node_index = 0;
                let mut index = i;
                loop {
                    if self.nodes[node_index].here {
                        table[index] += table[i];
                    }
                    if index >= design.len() {
                        break;
                    }
                    match self.nodes[node_index].children[COLOR_INDEX[design[index] as usize]] {
                        0 => break,
                        idx => { node_index = idx as usize; index += 1 }
                    }
                }
            }
        }
        table[design.len()]
    }
}

const fn mk_index() -> [usize; 128] {
    let mut index = [0; 128];
    index[b'w' as usize] = 1;
    index[b'u' as usize] = 2;
    index[b'b' as usize] = 3;
    index[b'r' as usize] = 4;
    index
}

const COLOR_INDEX: [usize; 128] = mk_index();
pub fn solve(input: &str) -> (u64, u64) {
    let mut stream = BitStream::from(input);

    let mut p1 = 0;
    let p2 = helper(&mut stream, &mut p1);

    (p1, p2)
}

fn helper(stream: &mut BitStream, p1: &mut u64) -> u64 {
    let version = stream.next_chunk::<3>();
    *p1 += version;
    let id = stream.next_chunk::<3>();
    if id == 4 {
        let mut value = 0;
        loop {
            let prefix = stream.next_chunk::<1>();
            let data = stream.next_chunk::<4>();
            value = value << 4 | data;
            if prefix == 0 {
                break;
            }
        }
        value
    } else {
        if stream.next_chunk::<1>() == 0 {
            let limit = stream.next_chunk::<15>() + stream.read;
            let it = std::iter::from_fn(|| {
                if stream.read >= limit {
                    None
                } else {
                    Some(helper(stream, p1))
                }
            });
            do_operation(id, it)
        } else {
            let nb_packets = stream.next_chunk::<11>();
            let it = (0..nb_packets).map(|_| helper(stream, p1));
            do_operation(id, it)
        }
    }
}

fn do_operation(id: u64, mut it: impl Iterator<Item=u64>) -> u64 {
    match id {
        0 => it.sum(),
        1 => it.product(),
        2 => it.min().unwrap(),
        3 => it.max().unwrap(),
        5 => {
            let x = it.next();
            let y = it.next();
            (x > y) as u64
        }
        6 => {
            let x = it.next();
            let y = it.next();
            (x < y) as u64
        }
        7 => {
            let x = it.next();
            let y = it.next();
            (x == y) as u64
        }
        _ => unreachable!()
    }
}

#[inline]
fn hexa_to_bits(c: u8) -> u8 {
    if c >= b'A' { c - b'A' + 10 } else { c - b'0' }
}

struct BitStream<'a> {
    iterator: std::str::Bytes<'a>,
    buffer: u64,
    buffer_size: usize,
    read: u64,
}

impl BitStream<'_> {
    fn from(s: &str) -> BitStream<'_> {
        BitStream { iterator: s.bytes(), buffer: 0, buffer_size: 0, read: 0 }
    }
    
    fn next_chunk<const N: usize>(self: &mut Self) -> u64 {
        while self.buffer_size < N {
            let c = self.iterator.next().unwrap();
            let bits = hexa_to_bits(c) as u64;
            self.buffer = (self.buffer << 4) | bits;
            self.buffer_size += 4;
        }

        self.buffer_size -= N;
        self.read += N as u64;
        (self.buffer >> self.buffer_size) & ((1 << N) - 1)
    }
}
use std::str::from_utf8;

fn next_password(mut pwd: [u8; 8]) -> [u8; 8] {
    if (b'g'..b'o').contains(&pwd[3]) {
        return complete(pwd, b'p');
    }
    if pwd[3] <= b'x' {
        let pwd2 = complete(pwd, pwd[3]);
        if pwd2 > pwd {
            return pwd2;
        }
        else if pwd[3] != b'x' {
            pwd[3] += 1;
            return pwd;
        }
    }
    for i in (0..3).rev() {
        if pwd[i] != b'z' {
            let mut c = pwd[i]+1;
            while matches!(c, b'i' | b'o'| b'l') {
                c+=1;
            }
            pwd[i] = c;
            for c in pwd.iter_mut().take(3).skip(i+1) {
                *c = b'a';
            }
           return complete(pwd, b'a');
        }
    }
    unreachable!();
}

fn complete(mut pwd: [u8;8], c: u8) -> [u8; 8] {
    pwd[3] = c;
    pwd[4] = c;
    pwd[5] = c+1;
    pwd[6] = c+2;
    pwd[7] = c+2;
    pwd
}

pub fn solve(input: &str) -> Option<(String, String)> {
    let pwd = input.trim().as_bytes().try_into().unwrap();
    let pwd = next_password(pwd);
    let p1 =  from_utf8(&pwd).map(|p| p.to_string()).unwrap();
    let pwd = next_password(pwd);
    let p2 =  from_utf8(&pwd).map(|p| p.to_string()).unwrap();
    Some((p1, p2))
}

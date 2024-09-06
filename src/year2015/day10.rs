// look-and-say  (Conway)
// http://njohnston.ca/2010/10/a-derivation-of-conways-degree-71-look-and-say-polynomial/

use anyhow::*;
use lazy_static::lazy_static;
use crate::util::times;

lazy_static! {
    static ref CONWAY_TABLE: [(&'static str, Vec<usize>); 92] = [
        ("1112", vec!(62)),
        ("1112133", vec!(63, 61)),
        ("111213322112", vec!(64)),
        ("111213322113", vec!(65)),
        ("1113", vec!(67)),
        ("11131", vec!(68)),
        ("111311222112", vec!(83, 54)),
        ("111312", vec!(69)),
        ("11131221", vec!(70)),
        ("1113122112", vec!(75)),
        ("1113122113", vec!(76)),
        ("11131221131112", vec!(81)),
        ("111312211312", vec!(77)),
        ("11131221131211", vec!(78)),
        ("111312211312113211", vec!(79)),
        ("111312211312113221133211322112211213322112", vec!(80, 28, 90)),
        ("111312211312113221133211322112211213322113", vec!(80, 28, 89)),
        ("11131221131211322113322112", vec!(80, 29)),
        ("11131221133112", vec!(74, 28, 91)),
        ("1113122113322113111221131221", vec!(74, 31)),
        ("11131221222112", vec!(71)),
        ("111312212221121123222112", vec!(72)),
        ("111312212221121123222113", vec!(73)),
        ("11132", vec!(82)),
        ("1113222", vec!(85)),
        ("1113222112", vec!(86)),
        ("1113222113", vec!(87)),
        ("11133112", vec!(88, 91)),
        ("12", vec!(0)),
        ("123222112", vec!(2)),
        ("123222113", vec!(3)),
        ("12322211331222113112211", vec!(1, 60, 28, 84)),
        ("13", vec!(4)),
        ("131112", vec!(27)),
        ("13112221133211322112211213322112", vec!(23, 32, 60, 28, 90)),
        ("13112221133211322112211213322113", vec!(23, 32, 60, 28, 89)),
        ("13122112", vec!(6)),
        ("132", vec!(7)),
        ("13211", vec!(8)),
        ("132112", vec!(9)),
        ("1321122112", vec!(20)),
        ("132112211213322112", vec!(21)),
        ("132112211213322113", vec!(22)),
        ("132113", vec!(10)),
        ("1321131112", vec!(18)),
        ("13211312", vec!(11)),
        ("1321132", vec!(12)),
        ("13211321", vec!(13)),
        ("132113212221", vec!(14)),
        ("13211321222113222112", vec!(17)),
        ("1321132122211322212221121123222112", vec!(15)),
        ("1321132122211322212221121123222113", vec!(16)),
        ("13211322211312113211", vec!(19)),
        ("1321133112", vec!(5, 60, 28, 91)),
        ("1322112", vec!(25)),
        ("1322113", vec!(26)),
        ("13221133112", vec!(24, 28, 91)),
        ("1322113312211", vec!(24, 28, 66)),
        ("132211331222113112211", vec!(24, 28, 84)),
        ("13221133122211332", vec!(24, 28, 67, 60, 28, 88)),
        ("22", vec!(60)),
        ("3", vec!(32)),
        ("3112", vec!(39)),
        ("3112112", vec!(40)),
        ("31121123222112", vec!(41)),
        ("31121123222113", vec!(42)),
        ("3112221", vec!(37, 38)),
        ("3113", vec!(43)),
        ("311311", vec!(47)),
        ("31131112", vec!(53)),
        ("3113112211", vec!(48)),
        ("3113112211322112", vec!(49)),
        ("3113112211322112211213322112", vec!(50)),
        ("3113112211322112211213322113", vec!(51)),
        ("311311222", vec!(46, 37)),
        ("311311222112", vec!(46, 54)),
        ("311311222113", vec!(46, 55)),
        ("3113112221131112", vec!(46, 56)),
        ("311311222113111221", vec!(46, 57)),
        ("311311222113111221131221", vec!(46, 58)),
        ("31131122211311122113222", vec!(46, 59)),
        ("3113112221133112", vec!(46, 32, 60, 28, 91)),
        ("311312", vec!(44)),
        ("31132", vec!(45)),
        ("311322113212221", vec!(52)),
        ("311332", vec!(37, 28, 88)),
        ("3113322112", vec!(37, 29)),
        ("3113322113", vec!(37, 30)),
        ("312", vec!(33)),
        ("312211322212221121123222113", vec!(35)),
        ("312211322212221121123222112", vec!(34)),
        ("32112", vec!(36))
    ];
}

fn freqs_size(freqs: &[u32]) -> u32 {
    freqs
    .iter()
    .zip(CONWAY_TABLE.iter())
    .map(|(f, (seq, _))| f * seq.len() as u32)
    .sum()
}

pub fn solve(input: &str) -> Result<(u32, u32)> {
    let sequence = input.trim();
    let init = CONWAY_TABLE
        .iter()
        .position(|(seq, _)| *seq == sequence)
        .ok_or_else(|| anyhow!("Pattern {sequence} is not found in Conway table"))?;

    let mut freqs = [0; 92];
    freqs[init] = 1;

    let freqs = times(40, freqs, |x| next(x));
    let p1 = freqs_size(&freqs);
    let freqs = times(10, freqs, |x| next(x));
    let p2 = freqs_size(&freqs);
    Ok((p1, p2))
}

fn next(freqs: &[u32]) -> [u32; 92] {
    let mut output = [0; 92];
    for (f, (_, evolve)) in freqs.iter().zip(CONWAY_TABLE.iter()) {
        for &idx in evolve {
            output[idx] += f;
        }
    }
    output
}
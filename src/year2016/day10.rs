//use ahash::{HashMap, HashMapExt};
use nohash_hasher::IntMap;
use crate::util::{iter::*, parser::*};

struct Bot<'a> {
    low: (&'a str, u32), 
    high: (&'a str, u32),
    chips: [u32; 2],
    amount: usize,
}

pub fn solve(input: &str) -> (u32, u32) {
    let mut p1 = 0;
    let mut p2 = 1;
    
    let mut tokens = input.split_ascii_whitespace();
    let mut gifts = Vec::new();
    let mut bots = IntMap::default();
    while let Some(token) = tokens.next() {
        if token == "value" {
            let (value, _, _, bot, bot_no) = tokens.next_tuple().unwrap();
            let value: u32 = value.try_unsigned().unwrap();
            let bot_no: u32 = bot_no.try_unsigned().unwrap();
            gifts.push(((bot, bot_no), value));
        } else { // token == "bot"
            let (giver, _, _, _, receiver1, nbr1, _, _, _, receiver2, nbr2) = tokens.next_tuple().unwrap();
            let giver: u32 = giver.try_unsigned().unwrap();
            let low = (receiver1, nbr1.try_unsigned().unwrap());
            let high = (receiver2, nbr2.try_unsigned().unwrap()); 
            bots.insert(giver, Bot {low, high, chips: [0, 0], amount: 0 });
        }
    }

    while let Some(((kind, number), value)) = gifts.pop() {
        if kind =="bot" {
            bots.entry(number).and_modify(|bot| {
                bot.chips[bot.amount] = value;
                bot.amount += 1;

                if bot.amount == 2 {
                    let low = bot.chips[0].min(bot.chips[1]);
                    let high = bot.chips[0].max(bot.chips[1]);
                    gifts.push((bot.low, low));
                    gifts.push((bot.high, high));
                    if low == 17 && high == 61 {
                        p1 = number;
                    }
                }

            });
        } else if number <= 2 {
            p2 *= value;
        }
    }

    (p1, p2)
}
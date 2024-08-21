use itertools::Itertools;
use lazy_static::lazy_static;

pub struct Boss {
    hp: i32,
    damage: i32,
    armor: i32,
}

struct Item {
    cost: i32,
    damage: i32,
    armor: i32,
}

lazy_static! {
    static ref items: Vec<Vec<Item>> = {
        vec!(
            vec! ( Item {cost: 8,   damage: 4, armor: 0}
                 , Item {cost: 10,  damage: 5, armor: 0}
                 , Item {cost: 25,  damage: 6, armor: 0}
                 , Item {cost: 40,  damage: 7, armor: 0}
                 , Item {cost: 74,  damage: 8, armor: 0}
                ),
            vec! ( Item {cost: 13,  damage: 0, armor: 1}
                 , Item {cost: 31,  damage: 0, armor: 2}
                 , Item {cost: 53,  damage: 0, armor: 3}
                 , Item {cost: 75,  damage: 0, armor: 4}
                 , Item {cost: 102, damage: 0, armor: 5}
                 , Item {cost: 0,   damage: 0, armor: 0}
                 ),
            vec! ( Item {cost: 25,  damage: 1, armor: 0}
                 , Item {cost: 50,  damage: 2, armor: 0}
                 , Item {cost: 100, damage: 3, armor: 0}
                 , Item {cost: 20,  damage: 0, armor: 1}
                 , Item {cost: 40,  damage: 0, armor: 2}
                 , Item {cost: 80,  damage: 0, armor: 3}
                 , Item {cost: 0,   damage: 0, armor: 0}
                 ),
            vec! ( Item {cost: 25,  damage: 1, armor: 0}
                 , Item {cost: 50,  damage: 2, armor: 0}
                 , Item {cost: 100, damage: 3, armor: 0}
                 , Item {cost: 20,  damage: 0, armor: 1}
                 , Item {cost: 40,  damage: 0, armor: 2}
                 , Item {cost: 80,  damage: 0, armor: 3}
                 , Item {cost: 0,   damage: 0, armor: 0}
                 )
        )
    };
}

pub fn parse(input: &str) -> Option<Boss> {
    let (line1, line2, line3) = input.lines().next_tuple()?;
    let hp = line1.split(' ').nth(2).and_then(|w| w.parse().ok())?;
    let damage = line2.split(' ').nth(1).and_then(|w| w.parse().ok())?;
    let armor = line3.split(' ').nth(1).and_then(|w| w.parse().ok())?;
    Some(Boss { hp, damage, armor})
}

fn is_player_win(gear: &Vec<&Item>, boss: &Boss) -> bool {
    let player_damage = gear.iter().map(|&item| item.damage).sum::<i32>();
    let player_damage = 1.max(player_damage - boss.armor);
    let player_armor = gear.iter().map(|&item| item.armor).sum::<i32>();
    let boss_damage = 1.max(boss.damage - player_armor);
    let nb_turns_to_win = (boss.hp - 1) / player_damage;
    let nb_turns_to_lose = 99 / boss_damage;
    nb_turns_to_win <= nb_turns_to_lose
}

fn possible_gears<'a>() -> impl Iterator<Item=Vec<&'a Item>> {    
    items.iter().multi_cartesian_product()
} 

pub fn part1(boss: &Boss) -> Option<i32> {
    possible_gears()
        .filter(|gear| is_player_win(gear, boss))
        .map(|gear| gear.iter().map(|&item| item.cost).sum::<i32>())
        .min()
}

pub fn part2(boss: &Boss) -> Option<i32> {
    possible_gears()
        .filter(|gear| !is_player_win(gear, boss))
        .map(|gear| gear.iter().map(|&item| item.cost).sum::<i32>())
        .max()
}
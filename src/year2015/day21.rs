use crate::util::parser::*;

struct Boss {
    hp: i32,
    damage: i32,
    armor: i32,
}

#[derive(Clone, Copy)]
struct Item {
    cost: i32,
    damage: i32,
    armor: i32,
}

const ITEMS: [Item; 18] = [
    Item {cost: 8,   damage: 4, armor: 0},
    Item {cost: 10,  damage: 5, armor: 0},
    Item {cost: 25,  damage: 6, armor: 0},
    Item {cost: 40,  damage: 7, armor: 0},
    Item {cost: 74,  damage: 8, armor: 0},
        
    Item {cost: 13,  damage: 0, armor: 1},
    Item {cost: 31,  damage: 0, armor: 2},
    Item {cost: 53,  damage: 0, armor: 3},
    Item {cost: 75,  damage: 0, armor: 4},
    Item {cost: 102, damage: 0, armor: 5},
    Item {cost: 0,   damage: 0, armor: 0},

    Item {cost: 25,  damage: 1, armor: 0},
    Item {cost: 50,  damage: 2, armor: 0},
    Item {cost: 100, damage: 3, armor: 0},
    Item {cost: 20,  damage: 0, armor: 1},
    Item {cost: 40,  damage: 0, armor: 2},
    Item {cost: 80,  damage: 0, armor: 3},
    Item {cost: 0,   damage: 0, armor: 0},
];

pub fn solve(input: &str) -> (i32, i32) {
    let [hp, damage, armor] = input
        .iter_unsigned()
        .next_chunk()
        .unwrap();
    let boss = Boss { hp, damage, armor};

    let mut p1 = i32::MAX;
    let mut p2 = i32::MIN;
    for &weapon in &ITEMS[0..5] {
        for &armor in &ITEMS[5..11] {
            for &item1 in &ITEMS[11..18] {
                for &item2 in &ITEMS[11..18] {
                    let total_cost = weapon.cost + armor.cost + item1.cost + item2.cost;
                    if is_player_win([weapon, armor, item1, item2], &boss) {
                        p1 = p1.min(weapon.cost + armor.cost + item1.cost + item2.cost);
                    } else {
                        p2 = p2.max(total_cost);
                    }
                }
            }
        }
    }
    (p1, p2)
}

fn is_player_win(gear: [Item; 4], boss: &Boss) -> bool {
    let player_damage = gear.iter().map(|item| item.damage).sum::<i32>();
    let player_damage = 1.max(player_damage - boss.armor);
    let player_armor = gear.iter().map(|item| item.armor).sum::<i32>();
    let boss_damage = 1.max(boss.damage - player_armor);
    let nb_turns_to_win = (boss.hp - 1) / player_damage;
    let nb_turns_to_lose = 99 / boss_damage;
    nb_turns_to_win <= nb_turns_to_lose
}
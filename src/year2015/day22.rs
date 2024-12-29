use itertools::Itertools;
use ahash::{HashSet, HashSetExt};
use crate::util::{heap::MinHeap, parser::*};

#[derive(PartialEq, Eq, Clone, Hash)]
struct State {
    player_hp: i16,
    boss_hp: i16,
    current_mana: i16,
    poison: u8,
    recharge: u8,
    shield: u8,
}

pub fn solve(input: &str) -> (i16, i16) {
    let (boss_hp, boss_damage) = input.iter_unsigned().collect_tuple().unwrap();

    let p1 = simulate::<false>(boss_hp, boss_damage);
    let p2 = simulate::<true>(boss_hp, boss_damage);
    (p1, p2)
}

pub fn simulate<const HM: bool>(boss_hp: i16, boss_damage: i16) -> i16 {
    let state = State {
        player_hp: 50,
        boss_hp,
        current_mana: 500,
        poison: 0,
        recharge: 0,
        shield: 0,
    };
    dijkstra::<HM>(boss_damage, &state)
}

fn apply_effects(state: &mut State) -> bool {
    if state.shield > 0 {
        state.shield -= 1;
    }
    if state.poison > 0 {
        state.poison -= 1;
        state.boss_hp -= 3;
    }
    if state.recharge > 0 {
        state.recharge -= 1;
        state.current_mana += 101;
    }
    state.boss_hp <= 0
}

fn dijkstra<const HM: bool>(boss_damage: i16, state: &State) -> i16 {
    let mut queue = MinHeap::new();
    let mut seen = HashSet::new();
    queue.push(0, state.clone());
    
    while let Some((consumed_mana, state)) = queue.pop() {
        if !seen.insert(state.clone()) {
            continue;
        }
        let hp = state.player_hp - (if HM {1} else {0});
        if hp <= 0 {
            continue;
        }
        let mut state = state.clone();
        state.player_hp = hp;
        if apply_effects(&mut state) {
            return consumed_mana;
        }
    
        if state.current_mana >= 53 {
            let mut next = state.clone();
            next.current_mana -= 53;
            next.boss_hp -= 4;
            if next.boss_hp <= 0 || apply_effects(&mut next) {
                return consumed_mana + 53;
            }
            if boss_turn(boss_damage, &mut next) {
                queue.push(consumed_mana + 53, next);
            }
        }

        if state.current_mana >= 73 {
            let mut next = state.clone();
            next.current_mana -= 73;
            next.boss_hp -= 2;
            next.player_hp += 2;
            apply_effects(&mut next);
            if boss_turn(boss_damage, &mut next) {
                queue.push(consumed_mana + 73, next);
            }
        }

        if state.current_mana >= 113 && state.shield <= 1 {
            let mut next = state.clone();
            next.current_mana -= 113;
            next.shield = 6;
            apply_effects(&mut next);
            if boss_turn(boss_damage, &mut next) {
                queue.push(consumed_mana + 113, next);
            }
        }

        if state.current_mana >= 173  && state.poison <= 1 {
            let mut next = state.clone();
            next.current_mana -= 173;
            next.poison = 6;
            apply_effects(&mut next);
            if boss_turn(boss_damage, &mut next) {
                queue.push(consumed_mana + 173, next);
            }
        }

        if state.current_mana >= 229  && state.recharge <= 1 {
            let mut next = state;
            next.current_mana -= 229;
            next.recharge = 5;
            apply_effects(&mut next);
            if boss_turn(boss_damage, &mut next) {
                queue.push(consumed_mana + 229, next);
            }
        }
    }
    unreachable!();
}

fn boss_turn(mut boss_damage: i16, state: &mut State) -> bool {
    if state.shield > 0 {
        boss_damage = (boss_damage - 7).max(1);
    }
    state.player_hp -= boss_damage;
    state.player_hp >= 0 || state.current_mana >= 53
}

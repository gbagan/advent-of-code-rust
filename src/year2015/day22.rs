use itertools::Itertools;
use std::collections::HashSet;
use crate::util::heap::MinHeap;

struct Config {
    boss_damage: i16,
    hard_mode: bool,
}

#[derive(PartialEq, Eq, Copy, Clone, Hash)]
struct State {
    player_hp: i16,
    boss_hp: i16,
    current_mana: i16,
    poison: u8,
    recharge: u8,
    shield: u8,
}

pub fn parse(input: &str) -> Option<(i16, i16)> {
    let (line1, line2) = input.lines().next_tuple()?;
    let boss_hp = line1.split(' ').nth(2).and_then(|s| s.parse().ok())?;
    let boss_damage = line2.split(' ').nth(1).and_then(|s| s.parse().ok())?;
    Some((boss_hp, boss_damage))
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

fn dijkstra(config: &Config, state: &State) -> Option<i16>{
    let mut queue = MinHeap::new();
    let mut visited = HashSet::new();
    queue.push(0, *state);
    
    while let Some((consumed_mana, state)) = queue.pop() {
        if !visited.insert(state) {
            continue;
        }
        let hp = state.player_hp - (if config.hard_mode {1} else {0});
        if hp <= 0 {
            continue;
        }
        let mut state = state.clone();
        state.player_hp = hp;
        if apply_effects(&mut state) {
            return Some(consumed_mana);
        }
    
        if state.current_mana >= 53 {
            let mut next = state.clone();
            next.current_mana -= 53;
            next.boss_hp -= 4;
            if next.boss_hp <= 0 || apply_effects(&mut next) {
                return Some(consumed_mana + 53);
            }
            if boss_turn(config, &mut next) {
                queue.push(consumed_mana + 53, next);
            }
        }

        if state.current_mana >= 73 {
            let mut next = state.clone();
            next.current_mana -= 73;
            next.boss_hp -= 2;
            next.player_hp += 2;
            apply_effects(&mut next);
            if boss_turn(config, &mut next) {
                queue.push(consumed_mana + 73, next);
            }
        }

        if state.current_mana >= 113 && state.shield <= 1 {
            let mut next = state.clone();
            next.current_mana -= 113;
            next.shield = 6;
            apply_effects(&mut next);
            if boss_turn(config, &mut next) {
                queue.push(consumed_mana + 113, next);
            }
        }

        if state.current_mana >= 173  && state.poison <= 1 {
            let mut next = state.clone();
            next.current_mana -= 173;
            next.poison = 6;
            apply_effects(&mut next);
            if boss_turn(config, &mut next) {
                queue.push(consumed_mana + 173, next);
            }
        }

        if state.current_mana >= 229  && state.recharge <= 1 {
            let mut next = state.clone();
            next.current_mana -= 229;
            next.recharge = 5;
            apply_effects(&mut next);
            if boss_turn(config, &mut next) {
                queue.push(consumed_mana + 229, next);
            }
        }
    }
    None
}

fn boss_turn(config: &Config, state: &mut State) -> bool {
    let mut damage = config.boss_damage;
    if state.shield > 0 {
        damage = (damage - 7).max(1);
    }
    state.player_hp -= damage;
    state.player_hp >= 0 || state.current_mana >= 53
}

pub fn solve(input: &(i16, i16), hard_mode: bool) -> Option<i16> {
    let (boss_hp, boss_damage) = *input;
    let config = Config { boss_damage, hard_mode };
    
    let state = State {
        player_hp: 50,
        boss_hp,
        current_mana: 500,
        poison: 0,
        recharge: 0,
        shield: 0,
    };
    dijkstra(&config, &state)
}

pub fn part1(input: &(i16, i16)) -> Option<i16> {
    solve(input, false)
}

pub fn part2(input: &(i16, i16)) -> Option<i16> {
    solve(input, true)
}
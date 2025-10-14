use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Debug)]
struct Player {
    hp: i32,
    mana: i32
}

#[derive(Debug)]
struct Boss {
    hp: i32,
    damage: i32
}

#[derive(Debug, PartialEq)]
struct Spell {
    name: String,
    mana: i32,
    damage: i32,
    armor: i32,
    hp: i32,
    stamina: i32,
    time: i32
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Fight {
    player_hp: i32,
    player_mana: i32,
    boss_hp: i32,
    mana_spent: i32,
    active_spells: Vec<(String, i32)>, 
    is_player_turn: bool
}

impl Ord for Fight {
    fn cmp(&self, other: &Self) -> Ordering {
        other.mana_spent.cmp(&self.mana_spent)
    }
}

impl PartialOrd for Fight {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let spells: Vec<Spell> = vec![
        Spell { name: String::from("magic_missile"), mana: 53, damage: 4, armor: 0, hp: 0, stamina: 0, time: 1 },
        Spell { name: String::from("drain"), mana: 73, damage: 2, armor: 0, hp: 2, stamina: 0, time: 1 },
        Spell { name: String::from("shield"), mana: 113, damage: 0, armor: 7, hp: 0, stamina: 0, time: 6 },
        Spell { name: String::from("poison"), mana: 173, damage: 3, armor: 0, hp: 0, stamina: 0, time: 6 },
        Spell { name: String::from("recharge"), mana: 229, damage: 0, armor: 0, hp: 0, stamina: 101, time: 5 }
    ];
    
    let initial_state = Fight {
        player_hp: 50,
        player_mana: 500,
        boss_hp: 58,
        mana_spent: 0,
        active_spells: Vec::new(),
        is_player_turn: true
    };

    let mut queue = BinaryHeap::new();
    queue.push(initial_state);

    let mut min_mana_to_win = i32::MAX;

    while let Some(mut current_state) = queue.pop() {
        if current_state.mana_spent >= min_mana_to_win {
            continue;
        }
        
        let mut current_armor = 0;
        let mut next_active_spells = Vec::new();

        for (spell_name, timer) in &current_state.active_spells {
            let spell = spells.iter().find(|s| &s.name == spell_name).unwrap();

            current_state.boss_hp -= spell.damage;
            current_state.player_mana += spell.stamina;
            if spell.armor > 0 {
                current_armor = spell.armor;
            }

            let next_timer = timer - 1;
            if next_timer > 0 {
                next_active_spells.push((spell_name.clone(), next_timer));
            }
        }

        current_state.active_spells = next_active_spells;

        if current_state.boss_hp <= 0 {
            min_mana_to_win = min_mana_to_win.min(current_state.mana_spent);
            continue;
        }

        if current_state.is_player_turn {
            for spell in &spells {
                if current_state.player_mana < spell.mana {
                    continue;
                }

                if spell.time > 1 && current_state.active_spells.iter().any(|(name, _)| name == &spell.name) {
                    continue;
                }

                let mut next_state = current_state.clone();
                next_state.is_player_turn = false;

                next_state.player_mana -= spell.mana;
                next_state.mana_spent += spell.mana;

                if spell.time == 1 {
                    next_state.boss_hp -= spell.damage;
                    next_state.player_hp += spell.hp;
                } else {
                    next_state.active_spells.push((spell.name.clone(), spell.time));
                }

                if next_state.boss_hp <= 0 {
                    min_mana_to_win = min_mana_to_win.min(next_state.mana_spent);
                } else {
                    queue.push(next_state);
                }
            }
        } else {
            let mut next_state = current_state.clone();
            next_state.is_player_turn = true;

            let damage_dealt = (9 - current_armor).max(1);

            next_state.player_hp -= damage_dealt;

            if next_state.player_hp > 0 {
                queue.push(next_state);
            }
        }
    }

    println!("Part one: {}", min_mana_to_win);

    let initial_state = Fight {
        player_hp: 50,
        player_mana: 500,
        boss_hp: 58,
        mana_spent: 0,
        active_spells: Vec::new(),
        is_player_turn: true
    };

    let mut queue = BinaryHeap::new();
    queue.push(initial_state);

    let mut min_mana_to_win = i32::MAX;

    while let Some(mut current_state) = queue.pop() {
        if current_state.mana_spent >= min_mana_to_win {
            continue;
        }

        if current_state.is_player_turn {
            current_state.player_hp -= 1;
            if current_state.player_hp <= 0 {
                continue;
            }
        }
        
        let mut current_armor = 0;
        let mut next_active_spells = Vec::new();

        for (spell_name, timer) in &current_state.active_spells {
            let spell = spells.iter().find(|s| &s.name == spell_name).unwrap();

            current_state.boss_hp -= spell.damage;
            current_state.player_mana += spell.stamina;
            if spell.armor > 0 {
                current_armor = spell.armor;
            }

            let next_timer = timer - 1;
            if next_timer > 0 {
                next_active_spells.push((spell_name.clone(), next_timer));
            }
        }

        current_state.active_spells = next_active_spells;

        if current_state.boss_hp <= 0 {
            min_mana_to_win = min_mana_to_win.min(current_state.mana_spent);
            continue;
        }

        if current_state.is_player_turn {
            for spell in &spells {
                if current_state.player_mana < spell.mana {
                    continue;
                }

                if spell.time > 1 && current_state.active_spells.iter().any(|(name, _)| name == &spell.name) {
                    continue;
                }

                let mut next_state = current_state.clone();
                next_state.is_player_turn = false;

                next_state.player_mana -= spell.mana;
                next_state.mana_spent += spell.mana;

                if spell.time == 1 {
                    next_state.boss_hp -= spell.damage;
                    next_state.player_hp += spell.hp;
                } else {
                    next_state.active_spells.push((spell.name.clone(), spell.time));
                }

                if next_state.boss_hp <= 0 {
                    min_mana_to_win = min_mana_to_win.min(next_state.mana_spent);
                } else {
                    queue.push(next_state);
                }
            }
        } else {
            let mut next_state = current_state.clone();
            next_state.is_player_turn = true;

            let damage_dealt = (9 - current_armor).max(1);

            next_state.player_hp -= damage_dealt;

            if next_state.player_hp > 0 {
                queue.push(next_state);
            }
        }
    }

    println!("Part two: {}", min_mana_to_win);
}

use std::fs::read_to_string;
use std::collections::HashMap;
use std::iter::once;
use std::cmp::max;
use regex::Regex;

fn read_input(p: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let content = read_to_string(p)?;

    Ok(
        content
            .lines()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
    )
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Stats {
    hp: i32,
    damage: i32,
    armor: i32
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Weapon {
    name: String,
    cost: i32,
    damage: i32,
    armor: i32
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Armor {
    name: String,
    cost: i32,
    damage: i32,
    armor: i32
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Ring {
    name: String,
    cost: i32,
    damage: i32,
    armor: i32
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Loadout {
    weapon: Weapon,
    armor: Option<Armor>,
    rings: Option<Vec<Ring>>
}

fn parse_input(input: Vec<String>) -> Result<Stats, Box<dyn std::error::Error>> {
    let mut vals: Vec<i32> = Vec::new();
    let re = Regex::new(": (\\d+)$").unwrap();

    for s in input {
        let c = re.captures(&s).unwrap();
        let v = c.get(1).map(|m| m.as_str().to_string().parse::<i32>()).unwrap()?;
        vals.push(v)
    }

    Ok(
        Stats {
            hp: vals[0],
            damage: vals[1],
            armor: vals[2]
        }
    )
}

fn equip(stats: &mut Stats, loadout: Loadout) -> &mut Stats {
    stats.damage += loadout.weapon.damage;
    stats.armor += loadout.weapon.armor;

    if let Some(armor) = loadout.armor {
        stats.damage += armor.damage;
        stats.armor += armor.armor;
    }

    if let Some(rings) = loadout.rings {
        rings
            .iter()
            .for_each(|r| {
                stats.damage += r.damage;
                stats.armor += r.armor;
            })
    }
    
    stats
}

fn fight(mut boss: Stats, mut player: Stats, loadout: Loadout) -> bool {
    equip(&mut player, loadout);
    
    let player_attack = max(1, player.damage - boss.armor);
    let boss_attack = max(1, boss.damage - player.armor);
    
    while player.hp > 0 && boss.hp > 0 {
        boss.hp -= player_attack;
        if boss.hp <= 0 {
            break
        }
        player.hp -= boss_attack;
    }
    
    player.hp > 0
}

fn cost(loadout: Loadout) -> i32 {
    let mut cost = 0;

    cost += loadout.weapon.cost;

    if let Some(armor) = loadout.armor {
        cost += armor.cost
    }

    if let Some(rings) = loadout.rings {
        rings
            .iter()
            .for_each(|r| cost += r.cost)
    }

    cost
}

fn main() {
    let input = read_input("../input").unwrap();
    let mut boss = parse_input(input).unwrap();
    let mut player_template = Stats {
        hp: 100,
        damage: 0,
        armor: 0
    };

    let weapons: Vec<Weapon> = vec![
        Weapon { name: String::from("dagger"), cost: 8, damage: 4, armor: 0 },
        Weapon { name: String::from("shortsword"), cost: 10, damage: 5, armor: 0 },
        Weapon { name: String::from("warhammer"), cost: 25, damage: 6, armor: 0 },
        Weapon { name: String::from("longsword"), cost: 40, damage: 7, armor: 0 },
        Weapon { name: String::from("greataxe"), cost: 74, damage: 8, armor: 0 }
    ];

    let armors: Vec<Option<Armor>> = vec![
        None,
        Some(Armor { name: String::from("leather"), cost: 13, damage: 0, armor: 1 }),
        Some(Armor { name: String::from("chainmail"), cost: 31, damage: 0, armor: 2 }),
        Some(Armor { name: String::from("splintmail"), cost: 53, damage: 0, armor: 3 }),
        Some(Armor { name: String::from("bandedmail"), cost: 75, damage: 0, armor: 4 }),
        Some(Armor { name: String::from("platemail"), cost: 102, damage: 0, armor: 5 })
    ];

    let rings: Vec<Ring> = vec![
        Ring { name: String::from("damage1"), cost: 25, damage: 1, armor: 0 },
        Ring { name: String::from("damage2"), cost: 50, damage: 2, armor: 0 },
        Ring { name: String::from("damage3"), cost: 100, damage: 3, armor: 0 },
        Ring { name: String::from("defense1"), cost: 20, damage: 0, armor: 1 },
        Ring { name: String::from("defense2"), cost: 40, damage: 0, armor: 2 },
        Ring { name: String::from("defense3"), cost: 80, damage: 0, armor: 3 }
    ];

    let mut loadouts: Vec<Loadout> = Vec::new();
    
    let armor_choices = once(None)
        .chain(armors);
    
    for w in &weapons {
        for a in armor_choices.clone() {
            loadouts.push(
                Loadout {
                    weapon: w.clone(),
                    armor: a.clone(),
                    rings: None
                }
            );

            for r in &rings {
                loadouts.push(
                    Loadout {
                        weapon: w.clone(),
                        armor: a.clone(),
                        rings: Some(vec![r.clone()])
                    }
                );
            }

            if rings.len() >= 2 {
                for i in  0..rings.len() {
                    for j in (i+1)..rings.len() {
                        loadouts.push(
                            Loadout {
                                weapon: w.clone(),
                                armor: a.clone(),
                                rings: Some(vec![rings[i].clone(), rings[j].clone()])
                            }
                        );
                    }
                }
            }
        }
    }

    let mut fights: HashMap<Loadout, bool> = HashMap::new();

    for l in loadouts {
        let player = player_template.clone();
        let outcome = fight(boss.clone(), player, l.clone());
        fights.insert(l, outcome);
    }

    let min_cost_to_win = fights
        .iter()
        .filter(|&(_loadout, &won)| won)
        .map(|(loadout, _won)| cost(loadout.clone()))
        .min()
        .unwrap_or(0);

    let max_cost_to_lose = fights
        .iter()
        .filter(|&(_loadout, &won)| !won)
        .map(|(loadout, _won)| cost(loadout.clone()))
        .max()
        .unwrap_or(0);
            

    println!("Min Cost to Win: {:?}", min_cost_to_win);
    println!("Max Cost to Lose: {:?}", max_cost_to_lose);
}


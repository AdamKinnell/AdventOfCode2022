use itertools::{Itertools};

enum Resource {
    Ore,
    Clay,
    Obsidian,
    _Geode
}

#[derive(PartialEq, Eq)]
enum Bot {
    Orebot,
    Claybot,
    Obsidianbot,
    Geodebot
}

#[derive(Copy, Clone)]
pub struct Blueprint {
    orebot_ore_cost: u16,
    claybot_ore_cost: u16,
    obsidanbot_ore_cost: u16,
    obsidanbot_clay_cost: u16,
    geodebot_ore_cost: u16,
    geodebot_obsidian_cost: u16,
}

#[derive(Copy, Clone)]
pub struct SimulationState {
    next_minute: u16,

    // Bots
    orebot_count: u16,
    claybot_count: u16,
    obsidianbot_count: u16,
    geodebot_count: u16,

    // Resources
    ore_count: u16,
    clay_count: u16,
    obsidian_count: u16,
    geode_count: u16
}

fn div_ceil(x: u16, d: u16) -> u16 {
    (x + d - 1) / d
}

impl SimulationState {

    pub fn new() -> SimulationState {
        SimulationState {
            next_minute: 1,
            orebot_count: 1,
            claybot_count: 0,
            obsidianbot_count: 0,
            geodebot_count: 0,
            ore_count: 0,
            clay_count: 0,
            obsidian_count: 0,
            geode_count: 0 }
    }

    fn minutes_to_gather_resource(&self, needed: u16, resource: Resource) -> Option<u16> {
        let current;
        let production_per_min;
        match resource {
            Resource::Ore      => {current = self.ore_count; production_per_min = self.orebot_count},
            Resource::Clay     => {current = self.clay_count; production_per_min = self.claybot_count},
            Resource::Obsidian => {current = self.obsidian_count; production_per_min = self.obsidianbot_count},
            Resource::_Geode    => {current = self.geode_count; production_per_min = self.geodebot_count},
        }

        if production_per_min == 0 {
            return None;
        }

        let remaining_ore_needed = needed.saturating_sub(current);
        let minutes_required = div_ceil(remaining_ore_needed, production_per_min as u16);
        Some(minutes_required)
    }

    fn minutes_until_can_build_bot(&self, blueprint: &Blueprint, bot: &Bot) -> Option<u16> {
        match bot {
            Bot::Orebot =>
                self.minutes_to_gather_resource(blueprint.orebot_ore_cost, Resource::Ore),
            Bot::Claybot =>
                self.minutes_to_gather_resource(blueprint.claybot_ore_cost, Resource::Ore),
            Bot::Obsidianbot => {
                let ore_time = self.minutes_to_gather_resource(blueprint.obsidanbot_ore_cost, Resource::Ore);
                let clay_time = self.minutes_to_gather_resource(blueprint.obsidanbot_clay_cost, Resource::Clay);
                if ore_time.is_none() || clay_time.is_none() {return None;}
                Some(ore_time.unwrap().max(clay_time.unwrap()))
            }
            Bot::Geodebot => {
                let ore_time = self.minutes_to_gather_resource(blueprint.geodebot_ore_cost, Resource::Ore);
                let obsidian_time = self.minutes_to_gather_resource(blueprint.geodebot_obsidian_cost, Resource::Obsidian);
                if ore_time.is_none() || obsidian_time.is_none() {return None;}
                Some(ore_time.unwrap().max(obsidian_time.unwrap()))
            } ,

        }
    }

    fn build_bot(&mut self, blueprint: &Blueprint, bot: &Bot) {
        match bot {
            Bot::Orebot => {
                self.orebot_count += 1;
                self.ore_count -= blueprint.orebot_ore_cost;
            },
            Bot::Claybot => {
                self.claybot_count += 1;
                self.ore_count -= blueprint.claybot_ore_cost;
            },
            Bot::Obsidianbot => {
                self.obsidianbot_count += 1;
                self.ore_count -= blueprint.obsidanbot_ore_cost;
                self.clay_count -= blueprint.obsidanbot_clay_cost;

            },
            Bot::Geodebot => {
                self.geodebot_count += 1;
                self.ore_count -= blueprint.geodebot_ore_cost;
                self.obsidian_count -= blueprint.geodebot_obsidian_cost;

            }
        }
    }

    // Check if there is any additional benefit in building a bot of the given type.
    // If we already have enough resource production to build any type of bot, then
    fn is_useful_to_build_bot(&self, blueprint: &Blueprint, bot: &Bot) -> bool {
        match bot {
            Bot::Orebot => {
                let ore_production = self.orebot_count;
                ore_production <= (blueprint.claybot_ore_cost).max(blueprint.obsidanbot_ore_cost).max(blueprint.geodebot_ore_cost)
            }
            Bot::Claybot => {
                let clay_production = self.claybot_count;
                clay_production <= blueprint.obsidanbot_clay_cost
            },
            Bot::Obsidianbot => {
                let obsidian_production = self.obsidianbot_count;
                obsidian_production <= blueprint.geodebot_obsidian_cost
            },
            Bot::Geodebot => true, // We can always use more geodes
        }
    }

    fn wait_for_and_build_bot(&self, blueprint: &Blueprint, bot: Bot) -> Option<SimulationState> {
        let mins_to_wait = self.minutes_until_can_build_bot(blueprint, &bot);
        mins_to_wait?;

        let mut new = *self;
        new.advance_time(mins_to_wait.unwrap() as u16 + 1); // Include 1 minute build time.
        new.build_bot(blueprint, &bot);
        Some(new)
    }

    fn advance_time(&mut self, minutes: u16) {
        self.ore_count += (self.orebot_count * minutes) as u16;
        self.clay_count += (self.claybot_count * minutes) as u16;
        self.obsidian_count += (self.obsidianbot_count * minutes) as u16;
        self.geode_count += (self.geodebot_count * minutes) as u16;
        self.next_minute += minutes;
    }
}

pub fn parse_blueprints(input: &str) -> impl Iterator<Item=Blueprint> + '_ {
    return input
        .lines()
        .map(|line| {
            let tokens = line.split(' ').collect_vec();
            Blueprint {
                orebot_ore_cost: tokens[6].parse().unwrap(),
                claybot_ore_cost: tokens[12].parse().unwrap(),
                obsidanbot_ore_cost: tokens[18].parse().unwrap(),
                obsidanbot_clay_cost: tokens[21].parse().unwrap(),
                geodebot_ore_cost: tokens[27].parse().unwrap(),
                geodebot_obsidian_cost: tokens[30].parse().unwrap(),
            }
        })
}

pub fn calculate_blueprint_quality(blueprint: &Blueprint, state: SimulationState, finish_at_minute: u16) -> u16 {
    // After 32 minutes have been simulated, how many geodes did we collect with this build order?
    assert!(state.next_minute <= finish_at_minute);
    if state.next_minute == finish_at_minute {
        return state.geode_count;
    }

    let bots = [
        Bot::Geodebot,
        Bot::Obsidianbot,
        Bot::Claybot,
        Bot::Orebot
    ];

    let mut could_build_any_bots = false;
    let mut highest_score = 0;
    for bot in bots {

        // Trim build orders that don't gain us anything
        if ! state.is_useful_to_build_bot(blueprint, &bot) {
            continue;
        }

        // What happens if we build this bot next?
        let new_state = state.wait_for_and_build_bot(blueprint, bot);
        if new_state.is_none() { continue; } // We can't build this bot by waiting any amount of time
        if new_state.unwrap().next_minute > finish_at_minute { continue; } // We can't build this bot in time

        // Continue to simulate if we assume this decision
        could_build_any_bots = true;
        let highest_score_from_this_path = calculate_blueprint_quality(blueprint, new_state.unwrap(), finish_at_minute);
        highest_score = highest_score.max(highest_score_from_this_path);
    }

    if could_build_any_bots {
        highest_score
    } else {
        // No time left to build any more bots, so let's just simulate resource collection until the end
        let minutes_remaining = finish_at_minute - state.next_minute;
        let mut end_state = state;
        end_state.advance_time(minutes_remaining);
        calculate_blueprint_quality(blueprint, end_state, finish_at_minute)
    }

}
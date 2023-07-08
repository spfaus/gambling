use rand::*;
use std::{thread::sleep, time::Duration};

const STARTING_FUNDS: u32 = 500;
const STARTING_BET: u32 = 5;
const ODDS: u32 = 2;
const WINRATE: f64 = 0.5;
const TIMESTEP: f32 = 0.;

fn main() {
    let mut rng = thread_rng();
    let mut funds = STARTING_FUNDS;
    let mut bet = STARTING_BET;

    while funds > 0 {
        println!(
            "current funds: {}\nbetting next: {}\nfunds gain if win: {}\n",
            funds,
            bet,
            bet * ODDS
        );

        sleep(Duration::from_secs_f32(TIMESTEP));

        if rng.gen_bool(WINRATE) {
            funds += bet * ODDS;
            bet = STARTING_BET;
            println!("WON\n");
        } else {
            funds -= bet;
            bet *= ODDS;
            println!("LOST\n");
        }

        sleep(Duration::from_secs_f32(TIMESTEP));
    }
}

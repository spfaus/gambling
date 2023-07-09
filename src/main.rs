use rand::*;
use std::{thread::sleep, time::Duration};

const STARTING_FUNDS: f64 = 500.;
const STARTING_BET: f64 = 5.;
const ODDS: f64 = 2.;
const WINRATE: f64 = 0.5;
const TIMESTEP: f64 = 0.5;

// TODO: Have X players make Y bets each and display statistics

fn main() {
    let mut rng = thread_rng();
    let mut funds = STARTING_FUNDS;
    let mut bet = STARTING_BET;

    while funds > 0. {
        println!(
            "current funds: {}\nbetting next: {}\nfunds gain if win: {}\n",
            funds,
            bet,
            bet * ODDS
        );

        sleep(Duration::from_secs_f64(TIMESTEP));

        if rng.gen_bool(WINRATE) {
            funds += bet * ODDS;
            bet = STARTING_BET;
            println!("WON\n");
        } else {
            funds -= bet;
            bet *= ODDS;
            println!("LOST\n");
        }

        bet = bet.min(funds);

        sleep(Duration::from_secs_f64(TIMESTEP));
    }

    println!("All funds are lost!");
}

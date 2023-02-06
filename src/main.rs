use guwah::{ErrCode, Settings};
use crate::ntwk::Ntwk;
use crate::sim::Sim;

mod ntwk;
mod sim;
mod utils;

fn main() {
    let exit_code = run();
    std::process::exit(exit_code);
}

fn run() -> i32 {
    // Get cmd args -> global settings
    let args = std::env::args();
    let rns_settings = match Settings::from_args(args) {
        Ok(settings) => settings,
        Err(ErrCode::ShowHelpSign) => {
            println!("help board");
            return ErrCode::Okay as i32;
        },
        Err(e) => {
            eprintln!("parse error");
            return e as i32;
        },
    };

    //dbg!(&rns_settings);

    // Open ntwk file and parse according to settings
    let ntwk = match Ntwk::from_file(rns_settings.ntwk_file()) {
        Ok(n) => n,
        Err(e) => {
            eprintln!("ntwk parse error");
            return e as i32;
        }
    };

    //dbg!(&ntwk);

    // Open sim file and parse according to settings
    let sims = match Sim::from_file(rns_settings.sim_file()) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("sim parse error");
            return e as i32;
        }
    };

    for (timestep, instr_set_set) in sims.into_timeline().enumerate() {
        if let Some(instr_set) = instr_set_set {
            for instr in instr_set.iter() {
            }
        }
    }


    ErrCode::Okay as i32
}
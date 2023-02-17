use guwah::{ErrCode, RnsSettings};
use std::process;

mod timeline;

fn main() {
    // Get cmd args -> global settings
    let args = std::env::args();
    let rns_settings = RnsSettings::from_args(args).unwrap_or_else(|err| match err {
        ErrCode::ShowHelpSign => {
            println!("Help Board :D (TODO)");
            process::exit(ErrCode::Okay as i32);
        }
        e => {
            let err = e as i32;
            eprintln!("Error: {err}");
            process::exit(err);
        }
    });

    //dbg!(&rns_settings);

    timeline::start(rns_settings);
}

impl timeline::Settings for RnsSettings {
    fn ntwk_file(&self) -> &String {
        &self.ntwk_filename
    }

    fn sim_file(&self) -> &String {
        &self.sim_filename
    }
}

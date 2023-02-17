use crate::{
    timeline::{
        msg::packet::Packet,
        ntwk::Ntwk,
        sim::{Sim, SimIntoTimeline},
    },
    ErrCode,
};

use core::{iter::Zip, ops::RangeFrom};

use self::msg::avl_tree::AvlTree;

mod msg;
mod ntwk;
mod sim;
mod utils;

pub fn start<T>(settings: T) -> i32
where
    T: Settings,
{
    // Open ntwk file and parse according to settings
    let ntwk = match Ntwk::from_file(settings.ntwk_file()) {
        Ok(n) => n,
        Err(e) => {
            eprintln!("ntwk parse error");
            return e as i32;
        }
    };

    //dbg!(&ntwk);

    // Open sim file and parse according to settings
    let sims = match Sim::from_file(settings.sim_file()) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("sim parse error");
            return e as i32;
        }
    };

    let tree: AvlTree<Packet> = AvlTree::new();
    timeline(0, &sims).for_each(|(timestep, maybe_instr_set)| {
        if let Some(instr_set) = maybe_instr_set {
            for instr in instr_set.iter() {
                match instr {
                    sim::instr::Instr::Msg(m) => {
                        let pack = Packet::Message(m.into());
                    }
                    sim::instr::Instr::Rep(r) => match r.msg_id() {
                        Some(_) => todo!(),
                        None => tree
                            .iter()
                            .for_each(|msg_wrap| println!("{}", &msg_wrap.get())),
                    },
                    sim::instr::Instr::EndSim(_) => todo!(),
                }

                // Add instructions to avl tree for fast search,
                // and to vec stack for in-order reports
            }
        }
    });

    ErrCode::Okay as i32
}

fn timeline(start_time: u32, sims: &Sim) -> Zip<RangeFrom<u32>, SimIntoTimeline<'_>> {
    (start_time..).zip(sims.into_timeline())
}

pub trait Settings {
    fn ntwk_file(&self) -> &String;
    fn sim_file(&self) -> &String;
}

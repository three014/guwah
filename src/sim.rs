use std::cell::RefCell;

use crate::utils;

use self::{file_utils::SimErrCode, instr::Instr};

mod file_utils;
mod instr;

const DEFAULT_NUM_INSTRS: usize = 10;

#[derive(Debug)]
pub struct InstrSetSet(RefCell<Vec<Instr>>);

impl InstrSetSet {
    const fn new() -> InstrSetSet {
        InstrSetSet(RefCell::new(Vec::new()))
    }
}

#[derive(Debug)]
pub struct Sim {
    instr_set: Vec<InstrSetSet>,
}

impl Sim {
    pub fn from_file(filename: &String) -> Result<Sim, SimErrCode> {
        let mut status = SimErrCode::Okay;

        let lines = match utils::read_lines(filename) {
            Ok(l) => l,
            Err(e) => {
                eprintln!("{e}");
                status = SimErrCode::BadFile;
                return Err(status);
            }
        };

        let mut sims = Sim {
            instr_set: Vec::with_capacity(DEFAULT_NUM_INSTRS),
        };

        let mut prev_timestamp: u32 = 0;
        let mut curr_idx: u32 = 0;
        for line in lines {
            let s = line.unwrap_or("".to_string());
            let s = utils::strip_comment(s);
            if s.is_empty() {
                continue;
            };

            // parse line and turn into sim instruction, then add to sim list
            //println!("{}", &s);

            let instr = match file_utils::parse_instr(&s) {
                Ok(i) => i,
                Err(_) => continue,
            };

            sims.insert(instr, &mut prev_timestamp, &mut curr_idx);
        }

        // Ensure that last item in list is an endSim instruction
        let item: Option<&InstrSetSet> = sims.instr_set.last();
        match item {
            Some(instr_set_set) => {
                let mut vec_ref = instr_set_set.0.borrow_mut();
                vec_ref.shrink_to_fit();
                if !vec_ref.last().unwrap().is_endsim() {
                    status = SimErrCode::MismatchEndSimToken
                };
            }
            None => status = SimErrCode::EmptyContents,
        }

        match status {
            SimErrCode::Okay => Ok(sims),
            _ => Err(status),
        }
    }

    fn insert(&mut self, instr: Instr, prev_timestamp: &mut u32, curr_idx: &mut u32) {
        if instr.timestamp() == *prev_timestamp {
            insert_helper(self, *curr_idx, instr);
        } else {
            *prev_timestamp = instr.timestamp();
            if (*curr_idx as usize) < self.instr_set.len() {
                let iss: &InstrSetSet = self.instr_set.get(*curr_idx as usize).unwrap();
                let vec_len = iss.0.borrow().len();
                let mut vec_ref = iss.0.borrow_mut();
                vec_ref.shrink_to(vec_len);
                *curr_idx += 1;
            }
            insert_helper(self, *curr_idx, instr);
        }
    }

    /// Returns an iterator over the simulation timeline.
    /// 
    /// The iterator's interal index mimics that of a clock.
    /// If there are no instruction sets at the current time/index,
    /// the iterator returns an empty `Option`. Otherwise, the iterator
    /// returns a reference to the instruction set.
    pub fn into_timeline<'a>(&'a self) -> SimIntoTimeline<'a> {
        SimIntoTimeline {
            sim: self,
            index: 0,
        }
    }
}

fn insert_helper(sims: &mut Sim, curr_idx: u32, instr: Instr) {
    let result = sims.instr_set.get(curr_idx as usize);
    match result {
        Some(instr_set_set) => {
            instr_set_set.0.borrow_mut().push(instr);
        }
        None => {
            let new_instr_set_set = InstrSetSet::new();
            new_instr_set_set.0.borrow_mut().push(instr);
            sims.instr_set.push(new_instr_set_set);
        }
    }
}

// impl Deref for Sim {
//     type Target = [InstrSetSet];

//     fn deref<'a>(&'a self) -> &'a Self::Target {
//         &self.instr_set
//     }
// }

// impl<'a> IntoIterator for &'a Sim {
//     type Item = Option<&'a InstrSetSet>;
//     type IntoIter = SimIntoTimeline<'a>;

//     fn into_iter(self) -> Self::IntoIter {
//         SimIntoTimeline {
//             sim: self,
//             index: 0,
//         }
//     }
// }

pub struct SimIntoTimeline<'a> {
    sim: &'a Sim,
    index: usize,
}

impl<'a> Iterator for SimIntoTimeline<'a> {
    type Item = Option<&'a InstrSetSet>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.sim.instr_set.get(self.index) {
            Some(rc) => rc,
            None => return None,
        };

        let ret: Self::Item;
        let curr_timestamp = result.0.borrow()[0].timestamp();
        if (curr_timestamp as usize) == self.index {
            ret = Some(result);
            self.index += 1;
        } else {
            ret = None;
        }

        Some(ret)
    }
}

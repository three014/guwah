use std::cell::RefCell;

use crate::utils;

use self::{instr::Instr, file_utils::SimErrCode};

mod file_utils;
mod instr;

const DEFAULT_NUM_INSTRS: usize = 10;

#[derive(Debug)]
pub struct Sim {
    instr_set: Vec<RefCell<Vec<Instr>>>,
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
            if status != SimErrCode::Okay {
                break;
            }

            let s = line.unwrap_or("".to_string());
            let s = utils::strip_comment(s);
            if s.is_empty() { continue };

            // parse line and turn into sim instruction, then add to sim list
            //println!("{}", &s);

            let instr = match file_utils::parse_instr(&s) {
                Ok(i) => i,
                Err(_) => continue
            };

            sims.insert(instr, &mut prev_timestamp, &mut curr_idx);
        }

        { // Take care of last item
            let x = sims.instr_set.last();
            let y = x.unwrap();
            let mut z = y.borrow_mut();
            z.shrink_to_fit();
        }

        match status {
            SimErrCode::Okay => Ok(sims),
            _ => Err(status)
        }
    }

    fn insert(&mut self, instr: Instr, prev_timestamp: &mut u32, curr_idx: &mut u32) {
        if instr.timestamp() == *prev_timestamp {
            insert_helper(self, *curr_idx, instr);
        }
        else {
            *prev_timestamp = instr.timestamp();
            let v = self.instr_set.get(*curr_idx as usize);
            match v {
                Some(r) => {
                    let vec_len = r.borrow().len();
                    let mut vec = r.borrow_mut();
                    vec.shrink_to(vec_len);
                    *curr_idx += 1;
                },
                None => ()
            }
            insert_helper(self, *curr_idx, instr);
        }
    }
}

fn insert_helper(sims: &mut Sim, curr_idx: u32, instr: Instr) {
    let result = sims.instr_set.get(curr_idx as usize);
    match result {
        Some(v) => {
            v.borrow_mut().push(instr);
        },
        None => {
            sims.instr_set.push(RefCell::new(Vec::new()));
            let nv = sims.instr_set.get(curr_idx as usize).unwrap();
            nv.borrow_mut().push(instr);
        }
    }
}

impl<'a> IntoIterator for &'a Sim {
    type Item = &'a RefCell<Vec<Instr>>;

    type IntoIter = SimIntoIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        SimIntoIterator {
            sim: self,
            index: 0,
        }
    }   
}

pub struct SimIntoIterator<'a> {
    sim: &'a Sim,
    index: usize,
}

impl<'a> Iterator for SimIntoIterator<'a> {
    type Item = &'a RefCell<Vec<Instr>>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.sim.instr_set.get(self.index) {
            Some(rc) => rc,
            None => return None,
        };
        self.index += 1;
        Some(result)
    }
}

// impl IntoIterator for Sim {
//     type Item = Rc<RefCell<Vec<Instr>>>;
//     type IntoIter = SimIntoIterator;

//     fn into_iter(self) -> Self::IntoIter {
//         SimIntoIterator {
//             sim: self,
//             index: 0
//         }
//     }
// }

// pub struct SimIntoIterator {
//     sim: Sim,
//     index: usize,
// }

// impl Iterator for SimIntoIterator {
//     type Item = Rc<RefCell<Vec<Instr>>>;

//     fn next(&mut self) -> Option<Self::Item> {
//         let result = match self.sim.instr_set.get(self.index) {
//             Some(rc) => {
//                 Rc::clone(rc)
//             },
//             None => return None,
//         };
//         self.index += 1;
//         Some(result)
//     }
// }

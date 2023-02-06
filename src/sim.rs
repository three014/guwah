use crate::utils;

use self::{file_utils::SimErrCode, instr::Instr};

mod file_utils;
mod instr;

const DEFAULT_NUM_INSTRS: usize = 10;

#[derive(Debug)]
pub struct InstrSetSet(Vec<Instr>);

impl InstrSetSet {
    fn new() -> InstrSetSet {
        InstrSetSet(Vec::new())
    }

    pub const fn iter<'a>(&'a self) -> InstrAsIter<'a> {
        InstrAsIter { instr_set: self, index: 0 }
    }

    fn push(&mut self, instr: Instr) {
        self.0.push(instr)
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn as_mut_vec(&mut self) -> &mut Vec<Instr> {
        &mut self.0
    }
}

pub struct InstrAsIter<'a> {
    instr_set: &'a InstrSetSet,
    index: usize,
}

impl<'a> Iterator for InstrAsIter<'a> {
    type Item = &'a Instr;

    fn next(&mut self) -> Option<Self::Item> {
        let vec_ref = &self.instr_set.0;
        if let Some(result) = vec_ref.get(self.index) {
            self.index += 1;
            Some(result)
        }
        else { None }
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

            let instr = match file_utils::parse_instr(&s) {
                Ok(i) => i,
                Err(_) => continue,
            };

            sims.insert(instr, &mut prev_timestamp, &mut curr_idx);
        }

        // Ensure that last item in list is an endSim instruction
        let item: Option<&mut InstrSetSet> = sims.instr_set.last_mut();
        match item {
            Some(instr_set_set) => {
                let vec_ref = instr_set_set.as_mut_vec();
                vec_ref.shrink_to_fit();
                // Should be okay to unwrap here
                // Vec couldn't be created unless there was an item to push to it
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
                let iss: &mut InstrSetSet = self.instr_set.get_mut(*curr_idx as usize).unwrap();
                let vec_len = iss.len();
                let vec_ref = iss.as_mut_vec();
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
    /// 
    /// # Examples
    /// 
    /// ``` 
    /// let sims = Sim::from_file("sample.sim");
    /// for (timestep, instr_set_set) in sims.into_timeline().enumerate() {
    ///     if let Some(instr_set) = instr_set_set {
    ///         for instr in instr_set.iter() {
    ///             assert!((instr.timestamp() as usize) == timestep);
    ///         }   
    ///     }
    /// }
    /// ```
    pub const fn into_timeline<'a>(&'a self) -> SimIntoTimeline<'a> {
        SimIntoTimeline {
            sim: self,
            index: 0,
            time: 0,
        }
    }
}

fn insert_helper(sims: &mut Sim, curr_idx: u32, instr: Instr) {
    let result = sims.instr_set.get_mut(curr_idx as usize);
    match result {
        Some(instr_set_set) => {
            instr_set_set.push(instr);
        }
        None => {
            let mut new_instr_set_set = InstrSetSet::new();
            new_instr_set_set.push(instr);
            sims.instr_set.push(new_instr_set_set);
        }
    }
}

pub struct SimIntoTimeline<'a> {
    sim: &'a Sim,
    index: usize,
    time: u32
}

impl<'a> Iterator for SimIntoTimeline<'a> {
    type Item = Option<&'a InstrSetSet>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.sim.instr_set.get(self.index) {
            Some(rc) => rc,
            None => return None,
        };

        let ret: Self::Item;
        let curr_timestamp = result.0[0].timestamp();
        if curr_timestamp == self.time {
            ret = Some(result);
            self.index += 1;
        } else {
            ret = None;
        }
        self.time += 1;

        Some(ret)
    }
}

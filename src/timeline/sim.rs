use self::{file_utils::SimErrCode, instr::Instr};
use crate::timeline::utils;

mod file_utils;
pub(crate) mod instr;

const DEFAULT_NUM_INSTRS: usize = 10;

#[derive(Debug)]
pub struct InstrSet(Vec<Instr>);

impl InstrSet {
    const fn new() -> InstrSet {
        InstrSet(Vec::new())
    }

    pub const fn iter<'a>(&'a self) -> InstrAsIter<'a> {
        InstrAsIter {
            instr_set: self,
            index: 0,
        }
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
    instr_set: &'a InstrSet,
    index: usize,
}

impl<'a> Iterator for InstrAsIter<'a> {
    type Item = &'a Instr;

    fn next(&mut self) -> Option<Self::Item> {
        let vec_ref = &self.instr_set.0;
        if let Some(result) = vec_ref.get(self.index) {
            self.index += 1;
            Some(result)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Sim {
    instr_set_set: Vec<InstrSet>,
}

impl Sim {
    pub fn from_file(filename: &String) -> Result<Sim, SimErrCode> {
        let mut status = SimErrCode::Okay;

        let lines = match utils::internal_utils::read_lines(filename) {
            Ok(lines) => lines,
            Err(e) => {
                eprintln!("{e}");
                status = SimErrCode::BadFile;
                return Err(status);
            }
        };

        let mut sims = Sim {
            instr_set_set: Vec::with_capacity(DEFAULT_NUM_INSTRS),
        };

        let mut prev_timestamp: u32 = 0;
        let mut curr_idx: u32 = 0;
        for line in lines {
            let s = line.unwrap_or("".to_string());
            let s = utils::internal_utils::strip_comment(s);
            if s.is_empty() {
                continue;
            };

            let instr = match file_utils::parse_instr(&s) {
                Ok(instr) => instr,
                Err(_) => continue,
            };

            sims.insert(instr, &mut prev_timestamp, &mut curr_idx);
        }

        // Ensure that last item in list is an endSim instruction
        let maybe_instr_set = sims.instr_set_set.last_mut();
        if let Some(instr_set) = maybe_instr_set {
            let vec_ref = instr_set.as_mut_vec();
            vec_ref.shrink_to_fit();
            // Should be okay to unwrap here
            // Vec couldn't be created unless there was an item to push to it
            if !vec_ref.last().unwrap().is_endsim() {
                status = SimErrCode::MismatchEndSimToken
            };
        } else {
            status = SimErrCode::EmptyContents
        }

        match status {
            SimErrCode::Okay => Ok(sims),
            _ => Err(status),
        }
    }

    fn insert(&mut self, instr: Instr, prev_timestamp: &mut u32, curr_idx: &mut u32) {
        if instr.timestamp() != *prev_timestamp {
            *prev_timestamp = instr.timestamp();
            if (*curr_idx as usize) < self.instr_set_set.len() {
                // Should be okay to unwrap, since all previous array entries
                // had to be initialized before insertion
                let iss: &mut InstrSet = self.instr_set_set.get_mut(*curr_idx as usize).unwrap();
                let vec_len = iss.len();
                let vec_ref = iss.as_mut_vec();
                vec_ref.shrink_to(vec_len);
                *curr_idx += 1;
            }
        }
        // Append new instruction to current array index
        insert_helper(self, *curr_idx, instr);
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
    /// Example using `enumerate`
    /// ```
    /// let sims = Sim::from_file("sample.sim");
    /// for (timestep, instr_set) in sims.into_timeline().enumerate() {
    ///     if let Some(instr_set) = instr_set {
    ///         for instr in instr_set.iter() {
    ///             assert!((instr.timestamp() as usize) == timestep);
    ///         }   
    ///     }
    /// }
    /// ```
    ///
    /// Example using `Iterator::for_each` and `zip` to compare timestamp with `u32` values
    /// ```
    /// let sims = Sim::from_file("sample.sim");
    /// let start: u32 = 0;
    /// (start..).zip(sims.into_timeline()).for_each(|(timestep, maybe_instr_set)| {
    ///     if let Some(instr_set) = maybe_instr_set {
    ///         for instr in instr_set.iter() {
    ///             assert!(instr.timestamp() == timestep);
    ///         }   
    ///     }
    /// });
    /// ```
    pub const fn into_timeline<'a>(&'a self) -> SimIntoTimeline<'a> {
        SimIntoTimeline {
            sim: self,
            index: 0,
            time: 0,
        }
    }
}

/// Appends a new instruction onto the current instruction set.
///
/// If the current instruction set is not initialized, the helper creates
/// a new instruction set, appends the new instruction to that set,
/// then appends that set to the Sim.
fn insert_helper(sims: &mut Sim, curr_idx: u32, instr: Instr) {
    let maybe_instr_set = sims.instr_set_set.get_mut(curr_idx as usize);
    if let Some(instr_set) = maybe_instr_set {
        instr_set.push(instr);
    } else {
        let mut new_instr_set = InstrSet::new();
        new_instr_set.push(instr);
        sims.instr_set_set.push(new_instr_set);
    }
}

pub struct SimIntoTimeline<'a> {
    sim: &'a Sim,
    index: usize,
    time: u32,
}

impl<'a> Iterator for SimIntoTimeline<'a> {
    type Item = Option<&'a InstrSet>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.sim.instr_set_set.get(self.index) {
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

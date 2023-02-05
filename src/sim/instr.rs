#[derive(Debug)]
pub struct MsgInstr {
    timestamp: u32,
    msg_id: u32,
    start_node: u32,
    end_node: u32,
}

#[derive(Debug)]
pub struct RepInstr {
    timestamp: u32,
    msg_id: Option<u32>,
}

#[derive(Debug)]
pub struct EndInstr(u32);

#[derive(Debug)]
pub enum Instr {
    Msg(MsgInstr),
    Rep(RepInstr),
    EndSim(EndInstr)
}

impl Instr {
    pub fn timestamp(&self) -> u32 {
        match self {
            Instr::Msg(m) => m.timestamp(),
            Instr::Rep(r) => r.timestamp(),
            Instr::EndSim(e) => e.timestamp(),
        }
    }

    pub fn is_endsim(&self) -> bool {
        match self {
            Instr::Msg(m) => m.is_endsim(),
            Instr::Rep(r) => r.is_endsim(),
            Instr::EndSim(e) => e.is_endsim(),
        }
    }
}

impl MsgInstr {
    pub fn new(timestamp: u32, msg_id: u32, start_node: u32, end_node: u32) -> MsgInstr {
        MsgInstr { timestamp, msg_id, start_node, end_node }
    }
    fn timestamp(&self) -> u32 { self.timestamp }
    fn is_endsim(&self) -> bool { false }
}

impl RepInstr {
    pub fn new(timestamp: u32, msg_id: Option<u32>) -> RepInstr {
        RepInstr { timestamp, msg_id }
    }
    fn timestamp(&self) -> u32 { self.timestamp }
    fn is_endsim(&self) -> bool { false }
}

impl EndInstr {
    pub fn new(timestamp: u32) -> EndInstr {
        EndInstr(timestamp)
    }
    fn timestamp(&self) -> u32 { self.0 }
    fn is_endsim(&self) -> bool { true }
}
use std::fmt::Display;

use crate::timeline::sim::instr::MsgInstr;

pub enum Packet {
    IcmpRequest(IcmpReplyPacket),
    IcmpReply(IcmpReqPacket),
    Message(MessagePacket),
}

pub struct MessagePacket {
    timestamp: u32,
    start_node: u32,
    current_node: u32,
    end_node: u32,
    id: u32,
    // Data
}

impl From<&MsgInstr> for MessagePacket {
    fn from(value: &MsgInstr) -> Self {
        MessagePacket {
            timestamp: value.timestamp(),
            start_node: value.start_node(),
            current_node: value.start_node(),
            end_node: value.end_node(),
            id: value.msg_id(),
        }
    }
}

pub struct IcmpReplyPacket {
    timestamp: u32,
    who_asked_node: u32,
    current_node: u32,
    queried_node: u32,
    node_found: bool,
}

pub struct IcmpReqPacket {
    timestamp: u32,
    who_asked_node: u32,
    current_node: u32,
    queried_node: u32,
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        todo!()
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

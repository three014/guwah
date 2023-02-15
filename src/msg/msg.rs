pub enum Msg {
    IcmpRequest,
    IcmpReply,
    Message,
}

pub struct Message {
    timestamp: u32,
    start_node: u32,
    current_node: u32,
    end_node: u32,
    // Data
}

pub struct IcmpReply {
    timestamp: u32,
    who_asked_node: u32,
    current_node: u32,
    queried_node: u32,
    node_found: bool,
}

pub struct IcmpRequest {
    timestamp: u32,
    who_asked_node: u32,
    current_node: u32,
    queried_node: u32,
}

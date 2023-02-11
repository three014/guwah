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
}
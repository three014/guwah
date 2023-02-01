use super::file_utils::NtwkErrCode;

#[derive(Debug)]
pub struct NtwkNode {
    id: u32,
    conn_count: usize,
    conn_list: Vec<u32>,
    queue_delay: u32,
}

impl NtwkNode {
    pub fn new(id: u32, conn_count: u32, queue_delay: u32) -> Result<NtwkNode, NtwkErrCode> {
        let mut create_node_status = NtwkErrCode::Okay;

        let n = NtwkNode {
            id,
            conn_count: conn_count.try_into().unwrap_or(10),
            conn_list: Vec::with_capacity(conn_count.try_into().unwrap_or_default()),
            queue_delay
        };
        if conn_count == 0 {
            create_node_status = NtwkErrCode::HasZeroConnections;
        }
        
        match create_node_status {
            NtwkErrCode::Okay => Ok(n),
            _ => Err(create_node_status)
        }
    }
    pub fn id(&self) -> u32 { self.id }
    pub fn conn_count(&self) -> usize { self.conn_count }
    pub fn conn_len(&self) -> usize { self.conn_list.len() }
    pub fn push(&mut self, conn: u32) {
        self.conn_list.push(conn);
    }
}

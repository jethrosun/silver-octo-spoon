use smoltcp::wire::*;

pub struct Flow<T>
where
    T: std::convert::AsRef<[u8]>,
{
    ip_endpoint: IpEndpoint,
    flow_content: Vec<TcpPacket<T>>,
}

impl<T> Flow<T>
where
    T: std::convert::AsRef<[u8]>,
{
    fn new() -> (&'static mut Self) {}
}

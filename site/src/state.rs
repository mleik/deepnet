use std::net::Ipv4Addr;
use ipnet::Ipv4Net;

#[derive(Default, PartialEq, Debug)]
pub struct State {
    pub core: CoreStateV0,
    pub charging: ChargingStateV0,
}

#[derive(Default, PartialEq, Debug)]
pub struct CoreStateV0 {
    pub networks: Vec<NetworkState>,
    pub devices: Vec<DeviceState>,
    pub tunnels: Vec<TunnelState>
}

#[derive(PartialEq, Debug)]
pub struct NetworkState {
    pub id: u64,
    pub subnet: Ipv4Net,
    pub service_ip: Ipv4Addr,
    pub dhcp_range: Option<(Ipv4Addr, Ipv4Addr)>,
}

#[derive(PartialEq, Debug)]
pub struct DeviceState {
    pub id: u64,
    pub network_id: u64,
    pub auth: AuthState,
    pub pre_auth: Vec<u16>,
}

#[derive(PartialEq, Debug)]
pub enum AuthState {
    Secret(String),
    Certificate(Vec<u8>),
    None,
}

#[derive(PartialEq, Debug)]
pub struct TunnelState {
    pub network_id: u64
    // TODO Find what IPSec needs.
}

#[derive(Default, PartialEq, Debug)]
pub struct ChargingStateV0 {
    // TODO learn more about charging
}
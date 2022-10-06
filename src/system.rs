use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Network {
    interface: String,
    received: u64,
    transmitted: u64,
}

#[derive(Debug, Serialize)]
pub struct Host {
    name: String,
    version: String,
    kernel: String,
    hostname: String,
}

pub fn init_info() -> Result<System, &'static str> {
    let mut sys = System::new_all();
	sys.refresh_all();
    Ok(sys)
}

pub fn get_network_info(sys: &System) -> Result<Vec<Network>, &'static str> {
    let mut nw = vec![];
    for (interface_name, data) in sys.networks() {
        nw.push(Network {
            interface: interface_name.to_string(),
            received: data.received(),
            transmitted: data.transmitted(),
        })
    }
    Ok(nw)
}

pub fn get_host_info(sys: &System) -> Result<Host, &'static str> {
	Ok(Host {
		name: sys.name().unwrap(),
		version: sys.os_version().unwrap(),
		kernel: sys.kernel_version().unwrap(),
		hostname: sys.host_name().unwrap(),
	})
}

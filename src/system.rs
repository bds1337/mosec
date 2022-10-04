use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};

pub fn get_host_info() -> Result<String, &'static str> {
	let mut sys = System::new_all();
	sys.refresh_all();

	println!("=> networks:");
	for (interface_name, data) in sys.networks() {
	    println!("{}: {}/{} B", interface_name, data.received(), data.transmitted());
	}

	println!("total memory: {} bytes", sys.total_memory());
	println!("used memory : {} bytes", sys.used_memory());
	println!("total swap  : {} bytes", sys.total_swap());
	println!("used swap   : {} bytes", sys.used_swap());

	println!("System name:             {:?}", sys.name());
	println!("System kernel version:   {:?}", sys.kernel_version());
	println!("System OS version:       {:?}", sys.os_version());
	println!("System host name:        {:?}", sys.host_name());

	Ok(sys.kernel_version().unwrap())
}
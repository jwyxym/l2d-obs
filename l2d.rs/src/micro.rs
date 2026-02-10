use crate::global::VOICE;
use cpal::{
	traits::{DeviceTrait, HostTrait, StreamTrait},
	Host,
	Device,
	default_host,
	StreamConfig,
	InputCallbackInfo
};
use std::{sync::atomic::Ordering::Relaxed, thread, time::Duration};
pub fn init() -> thread::JoinHandle<()> {
	thread::spawn(move || {
		let host: Host = default_host();
	
		let device: Device = host.default_input_device()
			.expect("没有找到可用的输入设备");
		
		if let Ok(desc) = device.description() {
			println!("输入设备: {}", desc.name());
		}
		
		if let Ok(config) = device.default_input_config() {
			let config: StreamConfig = config.into();
			if let Ok(stream) = device.build_input_stream(
				&config,
				move |data: &[f32], _: &InputCallbackInfo| {
					VOICE.store(if !data.is_empty() {
							data.iter()
								.map(|&x| x.abs())
								.sum::<f32>() / data.len() as f32 * 100.0
						} else {
							0.0
						}, Relaxed);
				},
				move |err| {
					eprintln!("{:?}", err);
				},
				None
			) {
				if let Ok(_) = stream.play() {
					loop {
						thread::sleep(Duration::from_secs(1));
					}
				}
			};
		}
	})
}
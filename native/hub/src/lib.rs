use bridge::respond_to_dart;
use gst::prelude::*;
use tokio_with_wasm::tokio;
use with_request::handle_request;

mod bridge;
mod messages;
mod with_request;

fn info() {
    gst::init()
        .map(|_| println!("GStreamer initialized."))
        .expect("Failed to initialize GStreamer.");
    println!("GStreamer version: {}", gst::version_string());
    let monitor = gst::DeviceMonitor::new();
    monitor.add_filter(Some("Video"), None);
    println!("Devices:");
    let devices = monitor.devices();
    devices.iter().for_each(|d| {
        println!("{} {}: {}", d.display_name(), d.name(), d.device_class());
        let caps = d.caps();
        if let Some(caps) = caps {
            println!("caps: {}", caps.to_string());
        }
    });
    println!("Providers:");
    monitor
        .providers()
        .iter()
        .map(|p| p.to_string())
        .collect::<Vec<String>>()
        .iter()
        .for_each(|s| println!("{}", s));
}
async fn main() {
    info();

    let mut request_receiver = bridge::get_request_receiver();

    // tokio::spawn(sample_functions::stream_mandelbrot());

    while let Some(request_unique) = request_receiver.recv().await {
        tokio::spawn(async {
            let response_unique = handle_request(request_unique).await;
            respond_to_dart(response_unique);
        });
    }
}

#[tokio::test]
async fn test() {
    main().await;
}

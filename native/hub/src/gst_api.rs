use gst::{bus::BusWatchGuard, prelude::*, sample, Fraction};
use gst_app::{AppSink, AppSinkCallbacks};

#[path = "run.rs"]
mod run;

#[derive(Debug, Clone)]
struct Device {
    name: String,
    display_name: String,
    device_class: String,
    // caps: String,
}

#[derive(Debug, Clone)]
pub struct Gst {
    version: (u32, u32, u32, u32), //GStreamer版本
    devices: Vec<Device>,          //设备列表
    providers: Vec<String>,        //设备提供商
    src: gst::Element,             //来源
    tee: gst::Element,             //分流器
    sink: gst::Element,            //接收
    pipeline: gst::Pipeline,       //管道
    bus: gst::Bus,                 //监听器
}
impl Gst {
    // 构造函数
    fn new() -> Self {
        gst::init()
            .map(|_| println!("GStreamer initialized."))
            .expect("Failed to initialize GStreamer.");
        println!("GStreamer version: {:?}", gst::version());
        Self {
            version: (0, 0, 0, 0),
            devices: Vec::new(),
            providers: Vec::new(),
            src: gst::ElementFactory::make_with_name("autovideosrc", Some("autovideosrc0"))
                .unwrap(),
            tee: gst::ElementFactory::make_with_name("tee", Some("tee0")).unwrap(),
            sink: gst::ElementFactory::make_with_name("appsink", Some("appsink0")).unwrap(),
            pipeline: gst::Pipeline::new(),
            bus: gst::Bus::new(),
        }
    }
    // 获取GStreamer版本
    pub fn get_ver(&mut self) -> (u32, u32, u32, u32) {
        self.version = gst::version();
        self.version
    }
    // 获取设备列表
    fn get_devices(&mut self, dev_type: &str) -> Vec<Device> {
        let monitor = gst::DeviceMonitor::new();
        monitor.add_filter(Some(dev_type), None);
        // 添加设备到Vec
        self.devices = monitor
            .devices()
            .iter()
            .map(|d| Device {
                name: d.name().to_string(),
                display_name: d.display_name().to_string(),
                device_class: d.device_class().to_string(),
            })
            .collect();
        self.devices.clone()
    }
    fn get_providers(&mut self, dev_type: &str) -> Vec<String> {
        let monitor = gst::DeviceMonitor::new();
        monitor.add_filter(Some(dev_type), None);
        self.providers = monitor.providers().iter().map(|p| p.to_string()).collect();
        self.providers.clone()
    }
    fn set_src(&mut self, name: &str) {
        self.src =
            gst::ElementFactory::make_with_name("avfvideosrc", Some("avfvideosrc0")).unwrap();
        self.src.set_property("name", name); //camera name such as "FaceTime HD Camera" or "HIK 720P CAMERA"
    }
    fn set_sink(&mut self, type_name: &str) {
        //appsink,glimagesink,osxvideosink
        self.sink =
            gst::ElementFactory::make_with_name(type_name, Some(&format!("{}{}", type_name, "0")))
                .unwrap();
    }
    fn link_src_to_sink(&mut self) {
        self.pipeline
            .add_many([&self.src, &self.tee, &self.sink])
            .unwrap();
        self.src.link(&self.sink).expect("Failed to link elements.");
    }
    fn set_pipeline_state(&mut self, state: gst::State) {
        self.pipeline
            .set_state(state)
            .map(|_| println!("Pipeline state start successful."))
            .expect("Unable to set the pipeline to the `Playing` state.");
    }

    fn set_bus_watcher(&mut self) -> Result<BusWatchGuard, glib::BoolError> {
        self.bus = self.pipeline.bus().unwrap();
        return self.bus.add_watch(move |_, msg| {
            match msg.view() {
                //流开始
                gst::MessageView::StreamStart(..) => {
                    println!("Stream started");
                    return glib::ControlFlow::Continue;
                }
                //流结束
                gst::MessageView::Eos(..) => {
                    println!("End of Stream");
                    return glib::ControlFlow::Continue;
                }
                //错误处理
                gst::MessageView::Error(err) => {
                    println!(
                        "Error received from element {}: {}",
                        msg.src()
                            .map(|s| String::from(s.path_string()))
                            .unwrap_or_else(|| String::from("None")),
                        err.error(),
                    );
                }
                _ => (),
            }
            glib::ControlFlow::Continue
        });
    }
    fn set_app_sink_callbacks(&mut self) {
        let appsink = self
            .sink
            .clone()
            .dynamic_cast::<AppSink>()
            .expect("Sink element is not an appsink");
        appsink.set_callbacks(
            AppSinkCallbacks::builder()
                .new_sample(on_new_sample)
                .build(),
        );
    }
}
// pub fn pipeline_
pub fn gst_main() {
    let mut gst = Gst::new();

    println!("Video/Source Devices:");
    gst.get_devices("Video/Source")
        .iter()
        .for_each(|d| println!("{:?}", d));

    println!("Video/Source Providers:");
    gst.get_providers("Video/Source")
        .iter()
        .for_each(|p| println!("{:?}", p));

    gst.set_src("HIK 720P CAMERA");
    gst.set_sink("appsink");
    gst.set_bus_watcher();
    gst.set_app_sink_callbacks();
    gst.link_src_to_sink();
    gst.set_pipeline_state(gst::State::Playing);

    let main_loop = glib::MainLoop::new(None, false);
    main_loop.run();
}

// 监听器

fn on_new_sample(appsink: &gst_app::AppSink) -> Result<gst::FlowSuccess, gst::FlowError> {
    if let Ok(sample) = appsink.pull_sample() {
        let caps = sample.caps().ok_or(gst::FlowError::Error)?;

        // 获取分辨率
        if let Some(structure) = caps.structure(0) {
            match (
                structure.get::<i32>("width"),
                structure.get::<i32>("height"),
                structure.get::<Fraction>("framerate"),
            ) {
                (Ok(width), Ok(height), Ok(framerate)) => {
                    let numerator = framerate.numer();
                    let denominator = framerate.denom();
                    let fps = numerator as f64 / denominator as f64;

                    println!("Current resolution: {}x{}, FPS: {}", width, height, fps);
                }
                _ => {
                    println!("Failed to get some information");
                }
            }
        }

        // 获取样本中的缓冲区
        if let Some(buffer) = sample.buffer() {
            // 获取缓冲区的元数据
            if let Ok(map_info) = buffer.map_readable() {
                // 获取元数据中的数据
                let data = map_info.as_slice();

                // 在这里进行对原始数据的处理
                println!("Received raw data with size: {}", data.len());
            }
            // buffer.remove_all_memory();
            // buffer.remove_all_memory();
        }
    }
    Ok(gst::FlowSuccess::Ok)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        run::run(gst_main);
    }
}

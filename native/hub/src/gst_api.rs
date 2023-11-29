use gst::prelude::*;
use gst::{bus::BusWatchGuard, prelude::*, sample, Fraction};
use gst_app::{AppSink, AppSinkCallbacks};

pub fn info() {
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

// #[path = "../run.rs"]
// mod run;
fn test_main() {
    info();
    let pipeline = gst::Pipeline::new(); //构建pipeline

    // let auto_video_src =
    // gst::ElementFactory::make_with_name("autovideosrc", Some("autovideosrc0")).unwrap(); //构建src
    // let avf_device_src0 =
    // gst::ElementFactory::make_with_name("avfdevice0", Some("avfvideosrc0")).unwrap(); //构建src

    let avf_device_src1 =
        gst::ElementFactory::make_with_name("avfvideosrc", Some("avfvideosrc1")).unwrap(); //构建src

    // avf_device_src1.set_property("device-index", 1);
    avf_device_src1.set_property("name", "HIK 720P CAMERA"); //按照名字打开指定摄像头

    // let uridecodebin =
    // gst::ElementFactory::make_with_name("uridecodebin", Some("uridecodebin0")).unwrap(); //构建uridecodebin

    // uridecodebin.set_property(
    //     "uri",
    //     "https://www.freedesktop.org/software/gstreamer-sdk/data/media/sintel_trailer-480p.webm",
    // ); //"file:///Users/bookshiyi/temp/test.mp4");

    let tee = gst::ElementFactory::make_with_name("tee", Some("tee0")).unwrap(); //构建tee,用于分流

    let app_sink = gst::ElementFactory::make_with_name("appsink", Some("appsink0")).unwrap(); //构建app_sink

    // let auto_video_sink =
    //     gst::ElementFactory::make_with_name("autovideosink", Some("autovideosink0")).unwrap(); //构建auto_video_sink

    let gl_sink = gst::ElementFactory::make_with_name("glimagesink", Some("glimagesink0")).unwrap(); //构建auto_video_sink
    let osx_sink =
        gst::ElementFactory::make_with_name("osxvideosink", Some("osxvideosink0")).unwrap(); //构建auto_video_sink

    pipeline
        .add_many([
            // &auto_video_src, //默认为摄像头
            &avf_device_src1, //默认为摄像头
            // &uridecodebin,    //多媒体文件
            // &tee, //分流
            &app_sink, //app_sink，用于app内部处理
                       // &auto_video_sink, //默认为显示器
                       // &gl_sink,  //OpenGL
                       // &osx_sink, //macOS
        ])
        .unwrap(); //添加到pipeline

    let appsink = app_sink
        .clone()
        .dynamic_cast::<AppSink>()
        .expect("Sink element is not an appsink");
    appsink.set_callbacks(
        AppSinkCallbacks::builder()
            .new_sample(on_new_sample)
            .build(),
    );

    avf_device_src1
        .link(&app_sink)
        .expect("Failed to link elements."); //src连接tee
                                             // uridecodebin.link(&tee).expect("Failed to link elements."); //uridecodebin连接tee
                                             // tee.link(&app_sink).expect("Failed to link elements."); //tee连接app_sink

    // tee.link(&auto_video_sink)
    //     .expect("Failed to link elements."); //tee连接video_sink

    // tee.link(&gl_sink).expect("Failed to link elements."); //tee连接video_sink
    // tee.link(&osx_sink).expect("Failed to link elements."); //tee连接video_sink

    let bus = pipeline.bus().unwrap(); //获取pipeline的bus
    let watcher = watcher(bus).unwrap(); //构建watcher
    println!("Watcher id: {:?}", watcher);

    //启动管道
    pipeline
        .set_state(gst::State::Playing)
        .map(|_| println!("Pipeline state start successful."))
        .expect("Unable to set the pipeline to the `Playing` state.");

    let main_loop = glib::MainLoop::new(None, false);
    main_loop.run();
}

// 监听器
fn watcher(bus: gst::Bus) -> Result<BusWatchGuard, glib::BoolError> {
    return bus.add_watch(move |_, msg| {
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
// fn app_sink_callback(app_sink: &gst_app::AppSink) -> Result<gst::FlowSuccess, gst::FlowError> {
//     app_sink.set_callbacks(
//         gst_app::AppSinkCallbacks::new()
//             .new_sample(|appsink| {
//                 let sample = appsink.pull_sample().ok_or(gst::FlowError::Eos)?;
//                 let caps = sample.get_caps().ok_or(gst::FlowError::Error)?;

//                 // 获取分辨率
//                 if let Some(structure) = caps.get_structure(0) {
//                     if let (Some(width), Some(height)) = (
//                         structure.get::<i32>("width"),
//                         structure.get::<i32>("height"),
//                     ) {
//                         println!("Current resolution: {}x{}", width, height);
//                     }
//                 }

//                 Ok(gst::FlowSuccess::Ok)
//             })
//             .build(),
//     );
// }
// fn main() {
//     run::run(test_main);
// }

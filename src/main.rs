use core::ffi::c_void;
use cpp::cpp;
use cstr::cstr;
use gst::prelude::*;
use qmetaobject::*;

// Define our own "GstPipeline" struct that wraps/contains the gstreamer pipeline(from the gstreamer crate)
// because QMetaObject requires the Default trait
// and we cannot implement a foreign trait(Default) on a foreign type(gst::Pipeline)
#[derive(QGadget, Clone)]
struct GstPipeline(gst::Pipeline); // <--- The (tuple) struct
impl Default for GstPipeline {
    fn default() -> Self {
        // Create a placeholder pipeline that is never meant to be used (because Rust doesn't have null)
        GstPipeline(gst::Pipeline::new(None))
    }
}

// Define a QObject that will contain the gstreamer pipeline
#[derive(QObject, Default)]
struct GstPipelineQObject {
    base: qt_base_class!(trait QObject),
    pipeline: qt_property!(GstPipeline), // <--- actual gstreamer pipeline will be inside our struct
    play: qt_method!(
        fn play(&self) {
            self.pipeline.0.set_state(gst::State::Playing).unwrap();
        }
    ),
}
impl GstPipelineQObject {
    // Take the gstreamer pipeline that will be used and instantiate it as a GstPipelineQObject QObject
    fn new(pipeline: gst::Pipeline) -> Self {
        let mut new = GstPipelineQObject::default();
        new.pipeline.0 = pipeline;
        new
    }
}

cpp! {{
    #include <gst/gst.h>
    #include <QQmlApplicationEngine>
    #include <QQuickWindow>
    #include <QQuickItem>
}}

fn main() {
    // Prepare gstreamer pipeline and elements
    gst::init().unwrap();
    let gst_pipeline = gst::Pipeline::new(None);
    let gst_source = gst::ElementFactory::make("videotestsrc", None).unwrap();
    let gst_glupload = gst::ElementFactory::make("glupload", None).unwrap();
    let gst_sink = gst::ElementFactory::make("qmlglsink", None).unwrap();
    gst_pipeline
        .add_many(&[&gst_source, &gst_glupload, &gst_sink])
        .unwrap();
    gst_source.link(&gst_glupload).unwrap();
    gst_glupload.link(&gst_sink).unwrap();

    // QMetaObject: Register QML types and create QML engine
    qml_register_type::<GstPipelineQObject>(cstr!("RustGstPipeline"), 1, 0, cstr!("GstPipeline"));
    let mut engine = QmlEngine::new();

    // Create a new GstPipelineQObject QObject by moving the gstreamer pipeline into it
    let q_pipeline = GstPipelineQObject::new(gst_pipeline);
    let q_pipeline_boxed = QObjectBox::new(q_pipeline);
    engine.set_object_property("gstPipeline".into(), q_pipeline_boxed.pinned());

    // Load UI file, set "widget" property on qmlglsink and run
    engine.load_file(QString::from("src/main.qml"));
    gst_sink.set_property("widget", get_video_item(&engine));
    engine.exec();
}

// Get GstGLVideoItem pointer from QmlEngine
// We can't do this in QMetaObject so we do it in C++
#[rustfmt::skip]
fn get_video_item(engine: &QmlEngine) -> *mut c_void {
    let engine_ptr: *mut c_void = engine.cpp_ptr();
    cpp!(unsafe[engine_ptr as "QQmlApplicationEngine *"] -> *mut c_void as "QQuickItem *"{
	QQuickWindow *rootObject = static_cast<QQuickWindow *> (engine_ptr->rootObjects().first());
	QQuickItem *videoItem = rootObject->findChild<QQuickItem *> ("videoItem");
	return videoItem;
    })
}

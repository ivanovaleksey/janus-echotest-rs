#[macro_use]
extern crate cstr_macro;
#[macro_use]
extern crate janus_plugin;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use std::os::raw::{c_int, c_char, c_void};
use std::sync::{Mutex, mpsc};
use std::ffi::CString;
use janus_plugin::{PluginCallbacks, PluginSession, RawPluginResult, PluginResult, PluginResultType,
    RawJanssonValue, JanssonValue};
use janus_plugin::sdp;

lazy_static! {
    static ref CHANNEL: Mutex<Option<mpsc::Sender<Message>>> = Mutex::new(None);
}

static mut GATEWAY: Option<&PluginCallbacks> = None;

#[derive(Debug)]
struct Message {
    handle: *mut PluginSession,
    transaction: *mut c_char,
    message: Option<JanssonValue>,
    jsep: Option<JanssonValue>,
}
unsafe impl std::marker::Send for Message {}

#[derive(Debug)]
struct EchoTestSession {
    field: u8,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum JsepKind {
    Offer { sdp: String },
    Answer { sdp: String },
}

const METADATA: janus_plugin::PluginMetadata = janus_plugin::PluginMetadata {
    version: 1,
    version_str: cstr!("0.1"),
    description: cstr!("EchoTest plugin"),
    name: cstr!("EchoTest plugin"),
    author: cstr!("Aleksey Ivanov"),
    package: cstr!("janus.plugin.echotest"),
};

extern "C" fn init(callback: *mut PluginCallbacks, config_path: *const c_char) -> c_int {
    janus_plugin::log(janus_plugin::LogLevel::Verb, "--> janus_echotest_init!!!");

    unsafe {
        let callback = callback.as_ref().unwrap();
        GATEWAY = Some(callback);
    }

    let (tx, rx) = mpsc::channel();
    *(CHANNEL.lock().unwrap()) = Some(tx);

    std::thread::spawn(move || { janus_echotest_handler(rx); });

    0
}

extern "C" fn destroy() {
    janus_plugin::log(janus_plugin::LogLevel::Verb, "--> janus_echotest_destroy!!!");
}

extern "C" fn create_session(handle: *mut PluginSession, error: *mut c_int) {
    janus_plugin::log(janus_plugin::LogLevel::Verb, "--> janus_echotest_create_session!!!");

    let handle = unsafe { &mut *handle };
    let mut session = EchoTestSession { field: 1 };

    handle.plugin_handle = &mut session as *mut EchoTestSession as *mut c_void;
}

extern "C" fn query_session(handle: *mut PluginSession) -> *mut RawJanssonValue {
    janus_plugin::log(janus_plugin::LogLevel::Verb, "--> janus_echotest_query_session!!!");
    std::ptr::null_mut()
}

extern "C" fn destroy_session(handle: *mut PluginSession, error: *mut c_int) {
    janus_plugin::log(janus_plugin::LogLevel::Verb, "--> janus_echotest_destroy_session!!!");
}

extern "C" fn handle_message(
    handle: *mut PluginSession,
    transaction: *mut c_char,
    message: *mut RawJanssonValue,
    jsep: *mut RawJanssonValue,
) -> *mut RawPluginResult {

    janus_plugin::log(janus_plugin::LogLevel::Verb, "--> janus_echotest_handle_message!!!");

    janus_plugin::log(janus_plugin::LogLevel::Verb, "--> acquiring transfer lock");
    let mutex = CHANNEL.lock().unwrap();
    let tx = mutex.as_ref().unwrap();
    janus_plugin::log(janus_plugin::LogLevel::Verb, "--> acquired transfer lock");

    let message = unsafe { JanssonValue::new(message) };
    let jsep = unsafe { JanssonValue::new(jsep) };

    let echo_message = Message {
        handle: handle,
        transaction: transaction,
        message: message,
        jsep: jsep,
    };
    janus_plugin::log(janus_plugin::LogLevel::Verb, "--> sending message to channel");
    tx.send(echo_message).expect("Sending to channel has failed");

    let result = PluginResult::new(PluginResultType::JANUS_PLUGIN_OK_WAIT, cstr!("Rust string"), None);
    result.into_raw()
}

extern "C" fn setup_media(handle: *mut PluginSession) {
    janus_plugin::log(janus_plugin::LogLevel::Verb, "--> janus_echotest_setup_media!!!");
}

extern "C" fn hangup_media(handle: *mut PluginSession) {
    janus_plugin::log(janus_plugin::LogLevel::Verb, "--> janus_echotest_hangup_media!!!");
}

extern "C" fn incoming_rtp(
    handle: *mut PluginSession,
    video: c_int,
    buf: *mut c_char,
    len: c_int,
) {
    let relay_fn = acquire_gateway().relay_rtp;
    relay_fn(handle, video, buf, len);
}

extern "C" fn incoming_rtcp(
    handle: *mut PluginSession,
    video: c_int,
    buf: *mut c_char,
    len: c_int,
) {}

extern "C" fn incoming_data(handle: *mut PluginSession, buf: *mut c_char, len: c_int) {}

extern "C" fn slow_link(handle: *mut PluginSession, uplink: c_int, video: c_int) {}

fn janus_echotest_handler(rx: mpsc::Receiver<Message>) {
    janus_plugin::log(janus_plugin::LogLevel::Verb, "Start handling thread");

    for received in rx.iter() {
        janus_plugin::log(janus_plugin::LogLevel::Verb, &format!("--> janus_echotest_handler, received: {:?}", received));

        if received.jsep.is_none() {
            janus_plugin::log(janus_plugin::LogLevel::Verb, "--> janus_echotest_handler, jsep is NONE, skipping");
            continue;
        }

        let jsep: JanssonValue = received.jsep.unwrap();
        let jsep_string: String = jsep.to_string(janus_plugin::JanssonEncodingFlags::empty());
        let jsep_json: JsepKind = serde_json::from_str(&jsep_string).unwrap();
        janus_plugin::log(janus_plugin::LogLevel::Verb, &format!("--> janus_echotest_handler, jsep: {:?}", jsep_json));

        let answer: serde_json::Value = match jsep_json {
            JsepKind::Offer { sdp } => {
                let offer: sdp::Sdp = sdp::Sdp::parse(CString::new(sdp).unwrap()).unwrap();
                janus_plugin::log(janus_plugin::LogLevel::Verb, &format!("--> janus_echotest_handler, offer: {:?}", offer));

                let answer: sdp::Sdp = answer_sdp!(offer);
                janus_plugin::log(janus_plugin::LogLevel::Verb, &format!("--> janus_echotest_handler, answer: {:?}", answer));

                let answer_str = answer.to_string();
                let sdp = answer_str.to_str().unwrap().to_owned();

                serde_json::to_value(JsepKind::Answer { sdp }).unwrap()
            }
            JsepKind::Answer { .. } => unreachable!(),
        };

        let event_json = json!({ "result": "ok" });
        let event_serde: JanssonValue = JanssonValue::from_str(&event_json.to_string(), janus_plugin::JanssonDecodingFlags::empty()).unwrap();
        let event: *mut RawJanssonValue = event_serde.as_mut_ref();

        let jsep_serde: JanssonValue = JanssonValue::from_str(&answer.to_string(), janus_plugin::JanssonDecodingFlags::empty()).unwrap();
        let jsep: *mut RawJanssonValue = jsep_serde.as_mut_ref();

        let push_event_fn = acquire_gateway().push_event;
        janus_plugin::get_result(push_event_fn(received.handle, &mut PLUGIN, received.transaction, event, jsep)).expect("Pushing event has failed");
    }
}

fn acquire_gateway() -> &'static PluginCallbacks {
    unsafe { GATEWAY }.expect("Gateway is NONE")
}

const PLUGIN: janus_plugin::Plugin = build_plugin!(
    METADATA,
    init,
    destroy,
    create_session,
    handle_message,
    setup_media,
    incoming_rtp,
    incoming_rtcp,
    incoming_data,
    slow_link,
    hangup_media,
    destroy_session,
    query_session
);

export_plugin!(&PLUGIN);

#[macro_use]
extern crate lazy_static;

mod janus;

use janus::{Plugin, Callback, PluginSession, PluginResult, json_t};
use std::os::raw::{c_int, c_char, c_void};
use std::sync::{Mutex, mpsc};
use std::ffi::CString;

lazy_static! {
    static ref CHANNELS: (Mutex<mpsc::Sender<Message>>, Mutex<mpsc::Receiver<Message>>) = {
        let (tx, rx) = mpsc::channel();
        (Mutex::new(tx), Mutex::new(rx))
    };

    static ref GATEWAY: Mutex<Option<Callback>> = Mutex::new(None);

    static ref ECHO_PLUGIN: Mutex<Plugin> = Mutex::new(Plugin::default());
}

const ECHOTEST_VERSION: u8 = 1;
const ECHOTEST_VERSION_STRING: &'static str = "0.1";
const ECHOTEST_DESCRIPTION: &'static str = "EchoTest plugin description";
const ECHOTEST_NAME: &'static str = "EchoTest plugin";
const ECHOTEST_AUTHOR: &'static str = "John";
const ECHOTEST_PACKAGE: &'static str = "janus.plugin.echotest";

#[derive(Debug)]
struct Message {
    handle: PluginSession,
    transaction: String,
    message: *mut json_t,
    jsep: *mut json_t,
}
unsafe impl std::marker::Send for Message {}

#[derive(Debug)]
struct EchoTestSession {
    field: u8,
}

#[no_mangle]
pub extern "C" fn create() -> *mut Plugin {
    let plugin = Plugin {
        init: Some(janus_echotest_init),
        destroy: Some(janus_echotest_destroy),
        get_api_compatibility: Some(janus_echotest_get_api_compatibility),
        get_version: Some(janus_echotest_get_version),
        get_version_string: Some(janus_echotest_get_version_string),
        get_description: Some(janus_echotest_get_description),
        get_name: Some(janus_echotest_get_name),
        get_author: Some(janus_echotest_get_author),
        get_package: Some(janus_echotest_get_package),
        create_session: Some(janus_echotest_create_session),
        handle_message: Some(janus_echotest_handle_message),
        setup_media: Some(janus_echotest_setup_media),
        incoming_rtp: Some(janus_echotest_incoming_rtp),
        incoming_rtcp: Some(janus_echotest_incoming_rtcp),
        incoming_data: None,
        slow_link: None,
        hangup_media: Some(janus_echotest_hangup_media),
        destroy_session: Some(janus_echotest_destroy_session),
        query_session: Some(janus_echotest_query_session),
    };

    let mut global_plugin = ECHO_PLUGIN.lock().unwrap();
    *global_plugin = plugin;

    Box::into_raw(Box::new(*global_plugin))
}

// Meta
extern "C" fn janus_echotest_get_api_compatibility() -> c_int {
    println!("RUST janus_echotest_get_api_compatibility!!!");
    janus::JANUS_PLUGIN_API_VERSION as c_int
}

extern "C" fn janus_echotest_get_version() -> c_int {
    println!("RUST janus_echotest_get_version!!!");
    ECHOTEST_VERSION as c_int
}

extern "C" fn janus_echotest_get_version_string() -> *const c_char {
    println!("RUST janus_echotest_get_version_string!!!");
    CString::new(ECHOTEST_VERSION_STRING)
        .unwrap()
        .into_raw()
}

extern "C" fn janus_echotest_get_description() -> *const c_char {
    println!("RUST janus_echotest_get_description!!!");
    CString::new(ECHOTEST_DESCRIPTION)
        .unwrap()
        .into_raw()
}

extern "C" fn janus_echotest_get_name() -> *const c_char {
    println!("RUST janus_echotest_get_name!!!");
    CString::new(ECHOTEST_NAME).unwrap().into_raw()
}

extern "C" fn janus_echotest_get_author() -> *const c_char {
    println!("RUST janus_echotest_get_author!!!");
    CString::new(ECHOTEST_AUTHOR).unwrap().into_raw()
}

extern "C" fn janus_echotest_get_package() -> *const c_char {
    println!("RUST janus_echotest_get_package!!!");
    CString::new(ECHOTEST_PACKAGE).unwrap().into_raw()
}
// End Meta

extern "C" fn janus_echotest_init(callback: *mut Callback, config_path: *const c_char) -> c_int {
    println!("RUST janus_echotest_init!!!");
    std::thread::spawn(|| { janus_echotest_handler(); });

    let callback = unsafe { *callback };
    let mut gateway = GATEWAY.lock().unwrap();
    *gateway = Some(callback);

    0
}

extern "C" fn janus_echotest_destroy() {
    println!("RUST janus_echotest_destroy!!!");
}

extern "C" fn janus_echotest_create_session(handle: *mut PluginSession, error: *mut c_int) {
    println!("RUST janus_echotest_create_session!!!");

    let handle = unsafe { &mut *handle };
    let mut session = EchoTestSession { field: 1 };

    handle.plugin_handle = &mut session as *mut EchoTestSession as *mut c_void;
}

extern "C" fn janus_echotest_query_session(handle: *mut PluginSession) -> *mut json_t {
    println!("RUST janus_echotest_query_session!!!");
    let json = json_t {
        type_: janus::json_type::JSON_NULL,
        refcount: 0,
    };

    Box::into_raw(Box::new(json))
}

extern "C" fn janus_echotest_destroy_session(handle: *mut PluginSession, error: *mut c_int) {
    println!("RUST janus_echotest_destroy_session!!!");
}

extern "C" fn janus_echotest_handle_message(
    handle: *mut PluginSession,
    transaction: *mut c_char,
    message: *mut json_t,
    jsep: *mut json_t,
) -> *mut PluginResult {

    println!("RUST janus_echotest_handle_message!!!");

    println!("RUST acquiring transfer lock");
    let tx = &CHANNELS.0.lock().unwrap();
    println!("RUST acquired transfer lock");

    let handle = unsafe { *handle };
    let transaction = unsafe {
        CString::from_raw(transaction)
            .into_string()
            .unwrap()
    };

    let echo_message = Message {
        handle: handle,
        transaction: transaction,
        message: message,
        jsep: jsep,
    };
    println!("RUST sending");
    tx.send(echo_message);

    unsafe {
        janus::janus_plugin_result_new(
            janus::janus_plugin_result_type::JANUS_PLUGIN_OK_WAIT,
            CString::new("Rust string").unwrap().into_raw(),
            std::ptr::null_mut::<json_t>(),
        )
    }
}

extern "C" fn janus_echotest_setup_media(handle: *mut PluginSession) {
    println!("RUST janus_echotest_setup_media!!!");
}

extern "C" fn janus_echotest_hangup_media(handle: *mut PluginSession) {
    println!("RUST janus_echotest_hangup_media!!!");
}

extern "C" fn janus_echotest_incoming_rtp(
    handle: *mut PluginSession,
    video: c_int,
    buf: *mut c_char,
    len: c_int,
) {
    println!("RUST janus_echotest_incoming_rtp!!!");
}

extern "C" fn janus_echotest_incoming_rtcp(
    handle: *mut PluginSession,
    video: c_int,
    buf: *mut c_char,
    len: c_int,
) {
    println!("RUST janus_echotest_incoming_rtcp!!!");
}

extern "C" {
    fn json_object() -> *mut json_t;
    fn json_object_get(object: *const json_t, key: *const c_char) -> *mut json_t;
    fn json_object_set_new(object: *mut json_t, key: *const c_char, value: *mut json_t) -> c_int;

    fn json_string(value: *const c_char) -> *mut json_t;
    fn json_string_value(string: *const json_t) -> *const c_char;

    fn json_pack(fmt: *const c_char, ...) -> *mut json_t;
}

extern "C" {
    fn janus_sdp_parse(sdp: *const c_char, error: *mut c_char, errlen: usize) -> *mut janus::janus_sdp;
    fn janus_sdp_generate_answer(offer: *mut janus::janus_sdp, ...) -> *mut janus::janus_sdp;
}

fn janus_echotest_handler() {
    println!("starting to handle");

    loop {
        println!("RUST acquiring receiver lock");
        let rx = &CHANNELS.1.lock().unwrap();
        println!("RUST acquired receiver lock");

        let received = rx.recv().unwrap();
        println!("RUST janus_echotest_handler, received: {:?}", received);

        if received.jsep.is_null() {
            println!("RUST janus_echotest_handler, jsep is NONE, skipping");
            continue;
        }

        let sdp =  unsafe {
            let jsep = received.jsep;
            let key = CString::new("sdp").unwrap().into_raw();
            let json_object = json_object_get(jsep, key);
            let c_string = json_string_value(json_object);

            CString::from_raw(c_string as *mut _)
        };

        println!("sdp: {:?}", sdp);

        let offer: janus::janus_sdp = unsafe {
            let bytes = vec![0u8; 512];
            let c_string = CString::from_vec_unchecked(bytes);

            let offer_prt: *mut janus::janus_sdp = janus_sdp_parse(sdp.into_raw(), c_string.into_raw(), 512);
            *offer_prt
        };
        println!("offer: {:?}", offer);

        let answer: janus::janus_sdp = unsafe {
            let answer_ptr: *mut janus::janus_sdp = janus_sdp_generate_answer(&offer as *const _ as *mut _, janus::JANUS_SDP_OA_DONE);
            *answer_ptr
        };
        println!("answer: {:?}", answer);

        let gateway = acquire_gateway().unwrap();
        println!("RUST gateway: {:?}", gateway);

        let push_event = gateway.push_event.unwrap();
        let plugin = *ECHO_PLUGIN.lock().unwrap();
        println!("RUST plugin: {:?}", plugin);

        let res: c_int = unsafe {
            let transaction = CString::new(received.transaction).unwrap().into_raw();

            let event = json_object();
            json_object_set_new(event, CString::new("echotest").unwrap().into_raw(), json_string(CString::new("event").unwrap().into_raw()));
            json_object_set_new(event, CString::new("result").unwrap().into_raw(), json_string(CString::new("ok").unwrap().into_raw()));

            let sdp_type = CString::new("answer").unwrap().into_raw();
            let sdp = janus::janus_sdp_write(&answer as *const _ as *mut _);
            let jsep = json_pack(
                CString::new("{ssss}").unwrap().into_raw(),
                CString::new("type").unwrap().into_raw(),
                sdp_type,
                CString::new("sdp").unwrap().into_raw(),
                sdp
            );

            push_event(&received.handle as *const _ as *mut _, &plugin as *const _ as *mut _, transaction, event, jsep)
        };

        println!("  >> Pushed event: {}", res);
    }
}

fn acquire_gateway() -> Option<Callback> {
    println!("RUST acquiring gateway lock");
    let rx = GATEWAY.lock().unwrap();
    println!("RUST acquired gateway lock");
    *rx
}

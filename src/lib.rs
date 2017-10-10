mod janus;

use janus::{Plugin, Callback, Session, PluginResult, json_t};
use std::os::raw::{c_int, c_char};

const ECHOTEST_VERSION: u8 = 1;
const ECHOTEST_VERSION_STRING: &'static str = "0.1";
const ECHOTEST_DESCRIPTION: &'static str = "EchoTest plugin description";
const ECHOTEST_NAME: &'static str = "EchoTest plugin";
const ECHOTEST_AUTHOR: &'static str = "John";
const ECHOTEST_PACKAGE: &'static str = "janus.plugin.echotest";

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
        incoming_rtp: None,
        incoming_rtcp: None,
        incoming_data: None,
        slow_link: None,
        hangup_media: Some(janus_echotest_hangup_media),
        destroy_session: Some(janus_echotest_destroy_session),
        query_session: Some(janus_echotest_query_session),
    };

    Box::into_raw(Box::new(plugin))
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
    std::ffi::CString::new(ECHOTEST_VERSION_STRING)
        .unwrap()
        .into_raw()
}

extern "C" fn janus_echotest_get_description() -> *const c_char {
    println!("RUST janus_echotest_get_description!!!");
    std::ffi::CString::new(ECHOTEST_DESCRIPTION)
        .unwrap()
        .into_raw()
}

extern "C" fn janus_echotest_get_name() -> *const c_char {
    println!("RUST janus_echotest_get_name!!!");
    std::ffi::CString::new(ECHOTEST_NAME).unwrap().into_raw()
}

extern "C" fn janus_echotest_get_author() -> *const c_char {
    println!("RUST janus_echotest_get_author!!!");
    std::ffi::CString::new(ECHOTEST_AUTHOR).unwrap().into_raw()
}

extern "C" fn janus_echotest_get_package() -> *const c_char {
    println!("RUST janus_echotest_get_package!!!");
    std::ffi::CString::new(ECHOTEST_PACKAGE).unwrap().into_raw()
}
// End Meta

extern "C" fn janus_echotest_init(callback: *mut Callback, config_path: *const c_char) -> c_int {
    println!("RUST janus_echotest_init!!!");
    0
}

extern "C" fn janus_echotest_destroy() {
    println!("RUST janus_echotest_destroy!!!");
}

extern "C" fn janus_echotest_create_session(handle: *mut Session, error: *mut c_int) {
    println!("RUST janus_echotest_create_session!!!");
}

extern "C" fn janus_echotest_query_session(handle: *mut Session) -> *mut json_t {
    println!("RUST janus_echotest_query_session!!!");
    let json = json_t {
        type_: janus::json_type::JSON_NULL,
        refcount: 0,
    };

    Box::into_raw(Box::new(json))
}

extern "C" fn janus_echotest_destroy_session(handle: *mut Session, error: *mut c_int) {
    println!("RUST janus_echotest_destroy_session!!!");
}

extern "C" fn janus_echotest_handle_message(
    handle: *mut Session,
    transaction: *mut c_char,
    message: *mut json_t,
    jsep: *mut json_t,
) -> *mut PluginResult {

    println!("RUST janus_echotest_handle_message!!!");

    let mut json = json_t {
        type_: janus::json_type::JSON_NULL,
        refcount: 0,
    };

    let result = PluginResult {
        type_: janus::janus_plugin_result_type::JANUS_PLUGIN_OK,
        text: std::ffi::CString::new("text").unwrap().into_raw(),
        content: &mut json,
    };

    Box::into_raw(Box::new(result))
}

extern "C" fn janus_echotest_setup_media(handle: *mut Session) {
    println!("RUST janus_echotest_setup_media!!!");
}

extern "C" fn janus_echotest_hangup_media(handle: *mut Session) {
    println!("RUST janus_echotest_hangup_media!!!");
}

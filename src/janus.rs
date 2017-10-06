#![allow(non_camel_case_types)]
include!("bindings.rs");

pub type Plugin = janus_plugin;
pub type Callback = janus_callbacks;
pub type Session = janus_plugin_session;
pub type PluginResult = janus_plugin_result;

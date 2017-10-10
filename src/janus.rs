#![allow(non_camel_case_types)]
include!("bindings.rs");

pub type Callback = janus_callbacks;
pub type Plugin = janus_plugin;
pub type PluginResult = janus_plugin_result;
pub type PluginSession = janus_plugin_session;

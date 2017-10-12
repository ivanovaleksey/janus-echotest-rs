#![allow(non_camel_case_types)]
include!("bindings/plugin.rs");
include!("bindings/sdp.rs");

pub type Callback = janus_callbacks;
pub type Plugin = janus_plugin;
pub type PluginResult = janus_plugin_result;
pub type PluginSession = janus_plugin_session;

impl Default for Plugin {
    fn default() -> Plugin {
        Plugin {
            init: None,
            destroy: None,
            get_api_compatibility: None,
            get_version: None,
            get_version_string: None,
            get_description: None,
            get_name: None,
            get_author: None,
            get_package: None,
            create_session: None,
            handle_message: None,
            setup_media: None,
            incoming_rtp: None,
            incoming_rtcp: None,
            incoming_data: None,
            slow_link: None,
            hangup_media: None,
            destroy_session: None,
            query_session: None,
        }
    }
}

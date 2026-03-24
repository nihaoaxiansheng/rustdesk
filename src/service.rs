use librustdesk::*;

#[cfg(not(target_os = "macos"))]
fn main() {}

#[cfg(target_os = "macos")]
fn main() {
    common::load_custom_client();
    apply_client_preset();
    hbb_common::init_log(false, "service");
    start_os_service();
}

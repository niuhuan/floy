use anyhow::Result;

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub async fn init_context(root: String) -> Result<()> {
    crate::domain::init_context(root.as_str()).await;
    Ok(())
}

#[flutter_rust_bridge::frb(sync)]
pub fn desktop_root() -> Result<String> {
    crate::commons::utils::desktop_root()
}

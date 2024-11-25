
pub mod components;
pub mod nodes;
pub mod renderers;
pub mod shaders;
pub mod util;
pub mod bev;
pub mod gli;

#[tokio::main]
async fn main() {
    bev::bev().await;
}


use std::path::Path;
use std::sync::Arc;
use fyrox::{
    core::pool::Handle,
    asset::manager::ResourceManager,
    scene::camera:: { SkyBox, SkyBoxBuilder },
    resource::texture::{ Texture, TextureWrapMode },
};


// The majority of this code was shamefully stolen from 

pub async fn request(resource_manager: &ResourceManager) -> SkyBox {

    let (front, back, left, right, top, bottom) = fyrox::core::futures::join!(
        resource_manager.request("data/textures/skybox-front.png"),
        resource_manager.request("data/textures/skybox-back.png"),
        resource_manager.request("data/textures/skybox-left.png"),
        resource_manager.request("data/textures/skybox-right.png"),
        resource_manager.request("data/textures/skybox-top.png"),
        resource_manager.request("data/textures/skybox-bottom.png")
    );

    let skybox = SkyBoxBuilder {
        front: front.ok(),
        back: back.ok(),
        left: left.ok(),
        right: right.ok(),
        top: top.ok(),
        bottom: bottom.ok()
    }.build().unwrap();

    let skybox_texture = skybox.cubemap().unwrap();
    let mut data = skybox_texture.data_ref();
    data.set_s_wrap_mode(TextureWrapMode::ClampToEdge);
    data.set_t_wrap_mode(TextureWrapMode::ClampToEdge);

    skybox

}


// This file is part of EXA.
// EXA is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as 
// published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
// EXA is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty
// of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
// You should have received a copy of the GNU General Public License along with EXA. If not, see 
// <https://www.gnu.org/licenses/>.

const FRAMEIMAGE_WIDTH: u16     = 480;
const FRAMEIMAGE_HEIGHT: u16    = 270;

use std::any::Any;

use tracing::instrument;
use { tracy_client, tracy_client_sys };
use fyrox:: {
    plugin::PluginContext,
    engine::InitializedGraphicsContext,
    renderer:: { 
        SceneRenderPass,
        RenderPassStatistics,
        framework::framebuffer:: { FrameBuffer, Attachment, AttachmentKind },
    }
};

#[instrument(skip(igc))]
pub fn frameimage_collect(igc: &mut InitializedGraphicsContext) {
    


}

pub struct FrameCollector {

    // Enable/disable this renderpass.
    enable:             bool,

}

impl FrameCollector {

    pub fn enable(&mut self) {
        self.enable = true;
    }

}

impl Default for FrameCollector {
    fn default() -> Self {
        FrameCollector {
            enable:         true,
        }
    }
}

impl SceneRenderPass for FrameCollector {
    
    fn on_ldr_render(
        &mut self,
        context: fyrox::renderer::SceneRenderPassContext,
    ) -> Result<fyrox::renderer::RenderPassStatistics, fyrox::renderer::framework::error::FrameworkError>
    {
        
        let statistics = RenderPassStatistics::default();
        
        if self.enable {
            let fbo = context.framebuffer;
            for attachment in fbo.color_attachments() {
                if attachment.kind == AttachmentKind::Color {

                    let texture = &attachment.texture.borrow().id();


                    // unsafe {
                    //     tracy_client_sys::___tracy_emit_frame_image(
                    //         image,
                    //         w,
                    //         h,
                    //         offset,
                    //         flip
                    //     )
                    // }

                }
            }
        }

        Ok(statistics)

    }
    
    fn source_type_id(&self) -> std::any::TypeId {
        self.type_id()
    }

}

enum FrameCollectorActivation {

    Inactive,

    Active {

        

    }

}

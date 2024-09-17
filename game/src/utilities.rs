
use fyrox::engine;


/// Call function `_f` while **engine::GraphicsContext** is subtype **GraphicsContext::InitializedGraphicsContext**.
pub fn with_igc<F>(
    graphics_context: &mut engine::GraphicsContext,
    mut function: F
)
    where F: FnMut(&engine::InitializedGraphicsContext) -> ()
{

    // Only while the graphics context *is initialized* should we retrieve it.
    if let engine::GraphicsContext::Initialized(igc) = graphics_context {
        function(igc)
    }

}

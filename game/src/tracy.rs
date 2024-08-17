
use tracing::instrument;
use fyrox:: {
    plugin::PluginContext,
    engine::InitializedGraphicsContext
};

#[instrument(skip(igc))]
pub fn frameimage_collect(igc: &mut InitializedGraphicsContext) {
    

    // 😳 Oh nooo, scawwy unswaf cowod...
    // Sadly, only OpenGL API calls will work for collecting frameimages at the current time.
    // Please refer to the following documents for more information about this process:
    // TODO 
    unsafe {
    }

}

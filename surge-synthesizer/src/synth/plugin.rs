crate::ix!();

#[derive(Debug)]
pub struct PluginLayer<'plugin_layer> {

    phantom: std::marker::PhantomData<&'plugin_layer i32>,
}

impl Default for PluginLayer<'plugin_layer> {
    fn default() -> Self {
        todo!();
    }
}

impl PluginLayerIF for PluginLayer<'plugin_layer> {

}

spawn_handle![PluginLayer,PluginLayerHandle,'plugin_layer];


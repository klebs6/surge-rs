crate::ix!();

#[derive(Debug)]
pub struct PluginLayer<'plugin_layer> {

    phantom: std::marker::PhantomData<&'plugin_layer i32>,
}

impl<'plugin_layer> Default for PluginLayer<'plugin_layer> {
    fn default() -> Self {
        todo!();
    }
}

impl<'plugin_layer> PluginLayerIF for PluginLayer<'plugin_layer> {

}

spawn_handle![PluginLayer,PluginLayerHandle,'plugin_layer];


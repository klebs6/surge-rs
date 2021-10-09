use crate::imports::*;
use crate::SampleHost;

#[no_mangle]
pub extern fn run_host() {

    // This is an example of a plugin being loaded. Change this to the appropriate path.
    let path = Path::new(
        "/Users/klebs/sunnyvale/work/repo/spenca_plugin/target/debug/libspenca_plugin.dylib",
    );

    // Create the host
    let host = Arc::new(Mutex::new(SampleHost));

    println!("Loading {}...", path.to_str().unwrap());

    // Load the plugin
    let mut loader = PluginLoader::load(path, Arc::clone(&host)).unwrap_or_else(
        |e| {
            panic!("Failed to load plugin: {}", e)
        },
    );

    // Create an instance of the plugin
    let mut instance = loader.instance().unwrap();

    // Get the plugin information
    let info = instance.get_info();

    println!(
        "Loaded '{}':\n\t\
              Vendor: {}\n\t\
              Presets: {}\n\t\
              Parameters: {}\n\t\
              VST ID: {}\n\t\
              Version: {}\n\t\
              Initial Delay: {} samples",
              info.name,
              info.vendor,
              info.presets,
              info.parameters,
              info.unique_id,
              info.version,
              info.initial_delay
    );

    // Initialize the instance
    instance.init();
    println!("Initialized instance!");

    let mut host = host.lock().unwrap();
    host.run(instance).unwrap();

    println!("Closing instance...");
    // Close the instance. This is not necessary as the instance is shut down when
    // it is dropped as it goes out of scope.
    // drop(instance);
}


use std::sync::Arc;
use parking_lot::Mutex;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use bevy::prelude::*;
use bevy::window::PresentMode;

// Internal types that won't be exposed to JS
#[derive(Default)]
struct InnerClient {
    app: Option<App>,
    #[cfg(feature = "vulkan")]
    vulkan_context: Option<vulkano::instance::Instance>,
}

#[derive(Default)]
struct ClientConfigInternal {
    width: u32,
    height: u32,
    title: String,
    vsync: bool,
}

// Configuration struct that will be exposed to JS
#[napi(object)]
#[derive(Clone)]
pub struct ClientConfig {
    pub width: u32,
    pub height: u32,
    pub title: String,
    pub vsync: bool,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            title: "Bevy App".to_string(),
            vsync: true,
        }
    }
}

impl From<ClientConfig> for ClientConfigInternal {
    fn from(config: ClientConfig) -> Self {
        Self {
            width: config.width,
            height: config.height,
            title: config.title,
            vsync: config.vsync,
        }
    }
}

#[napi]
pub struct DesktopClient(
    Arc<Mutex<InnerClient>>
);

impl Default for DesktopClient {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(InnerClient::default())))
    }
}

#[napi]
impl DesktopClient {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    #[napi]
    pub fn initialize(&self, js_config: Option<ClientConfig>) -> Result<()> {
        let mut inner = self.0.lock();
        let config: ClientConfigInternal = js_config.unwrap_or_default().into();
        
        let mut app = App::new();
        
        // Add default plugins based on build configuration
        #[cfg(any(feature = "dev", feature = "debug"))]
        {
            let window = Window {
                resolution: (config.width as f32, config.height as f32).into(),
                title: config.title.clone(),
                present_mode: if config.vsync { 
                    PresentMode::Fifo 
                } else { 
                    PresentMode::Immediate 
                },
                ..default()
            };
            app.add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(window),
                ..default()
            }));
        }

        #[cfg(feature = "pre")]
        app.add_plugins(DefaultPlugins);

        #[cfg(feature = "release")]
        app.add_plugins(MinimalPlugins);

        // Initialize Vulkan if the feature is enabled
        #[cfg(feature = "vulkan")]
        {
            let instance = vulkano::instance::Instance::new(vulkano::instance::InstanceCreateInfo {
                enabled_extensions: vulkano::instance::InstanceExtensions::none(),
                ..Default::default()
            })
            .map_err(|e| Error::from_reason(format!("Failed to initialize Vulkan: {}", e)))?;
            
            inner.vulkan_context = Some(instance);
        }

        inner.app = Some(app);
        Ok(())
    }

    #[napi]
    pub fn run(&self) -> Result<()> {
        let mut inner = self.0.lock();
        if let Some(mut app) = inner.app.take() {
            app.run();
            Ok(())
        } else {
            Err(Error::from_reason("App not initialized"))
        }
    }

    #[napi]
    pub fn cleanup(&self) -> Result<()> {
        let mut inner = self.0.lock();
        inner.app = None;
        #[cfg(feature = "vulkan")]
        {
            inner.vulkan_context = None;
        }
        Ok(())
    }
}

#[napi]
pub fn use_vulkan() -> bool {
    cfg!(feature = "vulkan")
}

#[napi]
pub fn get_build_mode() -> &'static str {
    if cfg!(feature = "dev") {
        "dev"
    } else if cfg!(feature = "debug") {
        "debug"
    } else if cfg!(feature = "pre") {
        "pre"
    } else if cfg!(feature = "release") {
        "release"
    } else {
        "unknown"
    }
}

#[napi]
pub fn create_client() -> DesktopClient {
    DesktopClient::new()
}

use tauri::{plugin::Plugin, Runtime};

pub struct MyPlugin<R: Runtime> {
    invoke_handler: Box<dyn Fn(tauri::Invoke<R>) + Send + Sync>,
}

impl<R: Runtime> MyPlugin<R> {
    pub fn new() -> Self {
        Self {
            invoke_handler: Box::new(tauri::generate_handler![]),
        }
    }
}

impl<R: Runtime> Plugin<R> for MyPlugin<R> {
    fn name(&self) -> &'static str {
        "my-plugin"
    }

    fn initialize(&mut self, _app: &tauri::AppHandle<R>, _config: serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn extend_api(&mut self, message: tauri::Invoke<R>) {
        (self.invoke_handler)(message)
    }
}

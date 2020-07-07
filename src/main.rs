use ksni::{MenuItem, Tray, TrayService};
use std::io::BufRead;

struct TrayWrapper(String);
impl Default for TrayWrapper {
    fn default() -> Self {
        TrayWrapper("".into())
    }
}

impl Tray for TrayWrapper {
    fn icon_name(&self) -> String {
        self.0.clone()
    }

    fn title(&self) -> String {
        self.0.clone()
    }

    fn menu(&self) -> Vec<MenuItem<Self>> {
        vec![]
    }
}

fn main() {
    let service = TrayService::new(TrayWrapper::default());
    let handle = service.handle();
    service.spawn();

    // Read the name from stdin and set the icon accordingly
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        if let Ok(file_name) = line {
            println!("Using {:#?}", &file_name);

            handle.update(|tray: &mut TrayWrapper| {
                tray.0 = file_name.clone();
            });
        }
    }
}

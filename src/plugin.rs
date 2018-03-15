use samp_sdk::consts::*;
use samp_sdk::types::Cell;
use samp_sdk::amx::{AmxResult, AMX};


define_native!(say);

pub struct HelloWorld {
    version: &'static str,
    amx_count: u32,
}

impl HelloWorld {
    pub fn load(&self) -> bool {
        log!("HelloWorld has loaded. Version: {}", self.version);
        return true;
    }

    pub fn unload(&self) {
        log!("Unloaded HelloWorld");
    }

    pub fn amx_load(&mut self, amx: &AMX) -> Cell {
        let natives = natives!(
            "rust_say" => say
        );

        match amx.register(&natives) {
            Ok(_) => log!("Natives are successful loaded"),
            Err(err) => log!("Whoops, there is an error {:?}", err),
        }

        self.amx_count += 1;

        AMX_ERR_NONE
    }

    pub fn amx_unload(&mut self, _: &AMX) -> Cell {
        self.amx_count -= 1;

        AMX_ERR_NONE
    }

    pub fn say(&mut self, _: &AMX) -> AmxResult<Cell> {
        log!("Hello world!");
        Ok(1)
    }
}

impl Default for HelloWorld {
    fn default() -> Self {
        HelloWorld {
            version: "0.0.1",
            amx_count: 0,
        }
    }
}

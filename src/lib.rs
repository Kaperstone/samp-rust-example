#[macro_use] extern crate samp_sdk;

mod plugin;
use plugin::HelloWorld;

new_plugin!(HelloWorld);

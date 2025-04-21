use crate::module_bindings::*;
use godot::prelude::*;
use godot::{global::godot_error, prelude::GodotClass};
use spacetimedb_sdk::{
    credentials, DbContext, Error, Event, Identity, Status, Table, TableWithPrimaryKey,
};
use std::sync::{Arc, Mutex};

const HOST: &str = "http://localhost:3000";
const DB_NAME: &str = "space-tag";

#[derive(GodotClass)]
#[class(base=Node)]
pub struct SpacetimeDBManager {
    ctx: DbConnection,
}

#[godot_api]
impl INode for SpacetimeDBManager {
    fn init(base: Base<Node>) -> Self {
        let ctx = DbConnection::builder()
            .with_uri(HOST)
            .with_module_name(DB_NAME)
            .build()
            .expect("Failed to connect to SpacetimeDB");

        // Start processing database events in a background thread
        ctx.run_threaded();
        let _ = ctx.reducers.say_hello();
        let _ = ctx.reducers.add("Hello, World!".to_string());
        Self { ctx }
    }
}

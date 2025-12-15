use bevy::prelude::*;
use std::thread;
use tokio::{
    runtime::Runtime,
    sync::mpsc::{UnboundedReceiver, unbounded_channel},
};

#[derive(Resource)]
struct NetReceiver(UnboundedReceiver<()>); // TODO: change () to sth meaningful

// bevy should listen to a receiver, tokio has the sender
pub fn receive_tokio(mut commands: Commands) {
    let (tx, rx) = unbounded_channel();
    commands.insert_resource(NetReceiver(rx));
    thread::spawn(move || {
        let rt = Runtime::new().expect("could not start tokio runtime");
        rt.block_on(async move {
            //
            // TODO: here is the async communication code main entry
            //
            tx
        });
    });
}

//! This is a sample OPC UA Client that connects to the specified server, fetches some
//! values before exiting.
extern crate opcua_types;
extern crate opcua_core;
extern crate opcua_client;

use std::path::PathBuf;

use opcua_client::prelude::*;

fn main() {
    // Logging is optional. If you call this, then you will see lots of output to the console.
    opcua_core::init_logging();

    // Use the sample client config to set up a client. The sample config has a number of endpoints
    // in it, one of which is marked as the default.
    let mut client = Client::new(ClientConfig::load(&PathBuf::from("../client.conf")).unwrap());

    // Create a session to the default endpoint
    if let Ok(session) = client.new_session_default() {
        let mut session = session.lock().unwrap();
        // Connect and do something with the server
        let result = connect(&mut session);
        if result.is_err() {
            println!("ERROR: Got an error - check this code {:?}", result.unwrap_err().description());
        }
    }
    else {
        println!("ERROR: Sample client cannot create a session!");
    }
}

fn connect(session: &mut Session) -> Result<(), StatusCode> {
    // Connect & activate the session.
    let _ = session.connect_and_activate_session()?;

    // Fetch some values from the sample server
    let read_nodes = vec![
        ReadValueId::read_value(NodeId::new_string(2, "v1")),
        ReadValueId::read_value(NodeId::new_string(2, "v2")),
        ReadValueId::read_value(NodeId::new_string(2, "v3")),
        ReadValueId::read_value(NodeId::new_string(2, "v4")),
    ];
    let data_values = session.read_nodes(&read_nodes)?.unwrap();

    // Print the values out
    println!("Values from server:");
    for data_value in data_values.iter() {
        if data_value.value.is_some() {
            println!("Value = {:?}", data_value.value.as_ref().unwrap());
        } else {
            println!("Value not found, error: {}", data_value.status.as_ref().unwrap().description());
        }
    }

    Ok(())
}


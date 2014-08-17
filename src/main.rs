#![feature(phase)]

extern crate sodiumoxide;
extern crate debug;
extern crate zmq;
extern crate capnp;
#[phase(plugin, link)] extern crate log;

mod capnp_zmq;

#[path="../target/out/"]
pub mod schema {
    pub mod request_capnp;
    pub mod reply_capnp;
}
mod client;
mod server;



static client_addr : &'static str = "tcp://localhost:5555";
static server_addr : &'static str = "tcp://*:5555";

fn main() {
    use server::server;
    use client::client;

    spawn(server);
    spawn(client);
    spawn(client);
    spawn(client);
    spawn(client);
}

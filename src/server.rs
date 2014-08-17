use abox = sodiumoxide::crypto::asymmetricbox;
use capnp;
use capnp::message::{MessageReader, MessageBuilder};
use capnp_zmq;
use std::str;
use zmq;

use schema::request_capnp;
use schema::reply_capnp;


pub fn server() {
    let mut ctx = zmq::Context::new();

    let mut socket = match ctx.socket(zmq::REP) {
        Ok(socket) => { socket },
        Err(e) => { fail!(e.to_string()) }
    };

    assert!(socket.bind(::server_addr).is_ok());

    info!("Listening on {}", ::server_addr);

    let (pub_key, sec_key) = abox::gen_keypair();

    loop {
        let frames = capnp_zmq::recv(&mut socket).unwrap();
        let segments = capnp_zmq::frames_to_segments(frames.as_slice());
        let reader = capnp::message::SegmentArrayMessageReader::new(
            segments.as_slice(),
            capnp::message::DefaultReaderOptions);
        let req = reader.get_root::<request_capnp::Request::Reader>();

        match str::from_utf8(req.get_data().as_slice()) {
            Some(s) => debug!("Received: {}", s),
            None => debug!("Received: some binary data"),
        };

        let req_key = abox::PublicKey::from_slice_by_ref(req.get_key()).unwrap();

        let nonce = abox::gen_nonce();
        let c = abox::seal(req.get_data(), &nonce, req_key, &sec_key);

        let mut message = capnp::message::MallocMessageBuilder::new_default();
        {
            let rep = message.init_root::<reply_capnp::Reply::Builder>();
            rep.set_data(c.as_slice());
            rep.set_key(pub_key.as_slice());
            rep.set_nonce(nonce.as_slice());
        }
        capnp_zmq::send(&mut socket, &mut message).unwrap();
    }
}

use abox = sodiumoxide::crypto::asymmetricbox;
use capnp;
use capnp::message::{MessageReader, MessageBuilder};
use capnp_zmq;
use std::io;
use std::str;
use std::time::duration::Duration;
use zmq;

use schema::request_capnp;
use schema::reply_capnp;


pub fn client() {
    info!("Connecting to {}", ::client_addr);

    let mut context = zmq::Context::new();
    let mut socket = context.socket(zmq::REQ).unwrap();

    assert!(socket.connect(::client_addr).is_ok());

    let (pub_key, sec_key) = abox::gen_keypair();

    let mut req_i = 0u32;

    loop {
        let req_str = format!("Request {}", req_i);
        debug!("Sending: {}", req_str);

        let mut message = capnp::message::MallocMessageBuilder::new_default();
        {
            let rep = message.init_root::<request_capnp::Request::Builder>();
            rep.set_data(req_str.as_bytes());
            rep.set_key({let abox::PublicKey(b) = pub_key; b});
        }
        capnp_zmq::send(&mut socket, &mut message).unwrap();

        let frames = capnp_zmq::recv(&mut socket).unwrap();
        let segments = capnp_zmq::frames_to_segments(frames.as_slice());
        let reader = capnp::message::SegmentArrayMessageReader::new(
            segments.as_slice(),
            capnp::message::DefaultReaderOptions);
        let rep = reader.get_root::<reply_capnp::Reply::Reader>();

        let rep_nonce = abox::Nonce::from_slice_by_ref(rep.get_nonce()).unwrap();
        let rep_key = abox::PublicKey::from_slice_by_ref(rep.get_key()).unwrap();

        match abox::open(rep.get_data(), rep_nonce, rep_key, &sec_key) {
            Some(data) => match str::from_utf8(data.as_slice()) {
                Some(s) => debug!("Received: {}", s),
                None => debug!("Received: some binary data"),
            },
            None => debug!("Received garbage"),
        };

        req_i += 1;
        io::timer::sleep(Duration::seconds(10));
    }
}


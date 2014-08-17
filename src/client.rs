use abox = sodiumoxide::crypto::asymmetricbox;
use capnp;
use capnp::message::{MessageReader, MessageBuilder, ReaderOptions};
use capnp::serialize_packed;

use std::io;
use std::str;
use std::time::duration::Duration;

use nanomsg::{NanoSocket};
use nanomsg::{AF_SP,NN_PAIR};

use schema::request_capnp;
use schema::reply_capnp;


pub fn client() {
    info!("Connecting to {}", ::client_addr);

    let sock = NanoSocket::new(AF_SP, NN_PAIR).unwrap();

    sock.connect(::client_addr).unwrap();

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

        let mut writer = io::MemWriter::new();
        serialize_packed::write_packed_message_unbuffered(&mut writer, &message).unwrap();
        sock.send(writer.unwrap().as_slice()).unwrap();

        let rep = sock.recv().unwrap();
        let mut reader = io::MemReader::new(rep);
        let reader = serialize_packed::new_reader_unbuffered(&mut reader, ReaderOptions::new()).unwrap();

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


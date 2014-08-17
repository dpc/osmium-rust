use abox = sodiumoxide::crypto::asymmetricbox;
use capnp;
use capnp::message::{MessageReader, MessageBuilder, ReaderOptions};
use capnp::serialize_packed;
use std::io;
use std::str;

use schema::request_capnp;
use schema::reply_capnp;

use nanomsg::{NanoSocket};
use nanomsg::{AF_SP,NN_PAIR};

pub fn server() {
    let sock = NanoSocket::new(AF_SP, NN_PAIR).unwrap();

    sock.bind(::server_addr).unwrap();

    info!("Listening on {}", ::server_addr);

    let (pub_key, sec_key) = abox::gen_keypair();

    loop {
        let req = sock.recv().unwrap();

        let mut reader = io::MemReader::new(req);
        let reader = serialize_packed::new_reader_unbuffered(&mut reader, ReaderOptions::new()).unwrap();
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

        let mut writer = io::MemWriter::new();
        serialize_packed::write_packed_message_unbuffered(&mut writer, &message).unwrap();
        sock.send(writer.unwrap().as_slice()).unwrap();
    }
}

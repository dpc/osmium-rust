@0xb39aa6bba22115d2;

# The reply has been encrypted with your public key, and
# with my secret key, which's public key and nonce I'm sending
# you here.
struct Reply {
	key @0 :Data;
	nonce @1 :Data;
	data @2 :Data;
}

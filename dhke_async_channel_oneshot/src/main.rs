#[allow(unused_must_use)]
mod dhke;
use oneshot::channel;
use std::thread;
fn main() {
        // Generate the public parameters prime (p) and generator (g) for both Alice and Bob
        let (prime, generator) = dhke::generate_shared_params();
        println!("Prime: {:?} and Generator: {:?}", prime, generator);

        // Generate Keypair for Alice and Bob
        let alice_keypair = dhke::Keypair::generate(&prime, &generator);
        let bob_keypair = dhke::Keypair::generate(&prime, &generator);


        println!( );
        println!("Alice: {:?}",alice_keypair);
        println!("Bob: {:?}",bob_keypair);


        /*
        create an async channel (single producer/single consumer) between Alice and Bob
        with Alice as sender and Bob as receiver using the oneshot module and send
        Alice's public key to Bob
         */
        let (alice, bob) = channel();
        let a = async_channel(alice,bob, alice_keypair.pk);
        println!("A message (Alice's public key) from Alice to Bob: {}", a);

        /* create an async channel (single producer/single consumer) between Bob and Alice
           with Bob as sender and Alice as receiver using the crate oneshot and send
           Bob's public key to Alice
         */
        let (bob, alice) = channel();
        let b = async_channel(bob,alice,bob_keypair.pk);
        println!("A message (Bob's public key) from Bob to Alice: {}", b);

        /*
        Alice computes her shared secret key from the received Bob's public key (b)
         and her private key (alice.sk)
         */
        let alice_shared_secret = dhke::generate_shared_secret(&prime, alice_keypair.sk, b);

        /*
        Bob computes his shared secret key from the received Alice's public key (a)
        and her private key (bob.sk)
         */
        let bob_shared_secret = dhke::generate_shared_secret(&prime, bob_keypair.sk, a);

        // Verify shared secret keys computed by both parties
        println!();
        println!("Alice's shared secret {:?}", alice_shared_secret.clone());
        println!("Bob's shared secret {:?}", bob_shared_secret.clone());
        assert_eq!(alice_shared_secret, bob_shared_secret);

        println!();
        println!("Done.")
    }

use  num_bigint::BigUint;
use oneshot::*;
// A function which uses oneshot channel to send a message from Alice to Bob and vice versa
fn async_channel(sender: oneshot::Sender<BigUint>, receiver: oneshot::Receiver<BigUint>, pub_key: BigUint) -> BigUint{
        thread::spawn(move || {
                sender.send(pub_key);
        });

        let message = receiver.recv().expect("Sender does not want to talk :(");

        return message;
}

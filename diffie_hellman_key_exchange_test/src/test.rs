mod lib;
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn diffie_hellman_shared_secret_matches() {
        let (prime, generator) = generate_shared_params();
        println!("Prime: {:?} Generator: {:?}", prime, generator);

        let alice = Keypair::generate(&prime, &generator);
        let bob = Keypair::generate(&prime, &generator);

        println!();
        println!("Alice: {:?}", alice);
        println!("Bob: {:?}", bob);

        let alice_shared_secret = generate_shared_secret(&prime, alice.sk, bob.pk);
        let bob_shared_secret = generate_shared_secret(&prime, bob.sk, alice.pk);

        println!();
        println!("Alice's shared secret {:?}", alice_shared_secret.clone());
        println!("Bob's shared secret {:?}", bob_shared_secret.clone());
        assert_eq!(alice_shared_secret, bob_shared_secret);

        println!();
        println!("Done.")
    }
}

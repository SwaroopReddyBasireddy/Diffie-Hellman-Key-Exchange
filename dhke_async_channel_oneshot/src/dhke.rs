#[allow(unused_imports)]
#[allow(unused_assignments)]
use num_bigint::{BigUint,RandBigInt,BigInt};
use glass_pumpkin::{safe_prime,prime};

#[derive(Debug)]

// A structure datatype for the key pair (secret key and public key)
pub struct Keypair{
    pub sk: BigUint,
    pub pk: BigUint,
}

// Implementation block for the struct Keypair to generate the keypair for a user
impl Keypair {
    pub fn generate(p: &BigUint, g: &BigUint) -> Keypair {
        let mut rng = rand::thread_rng();
        let secret_key= rng.gen_biguint(128);
        let public_key = BigUint::modpow(&g,&secret_key,&p);
        Keypair { sk: secret_key, pk: public_key }
    }
}

/*
Key Generation protocol: To generate the public parameters called prime modulus (p) and generator (g)
of datatype BigUint which is a member of the module "num_bigint". p is a safe prime such the q = (p-1)/2
is also a prime generated from num_bigint::BigUint. The generator g = 2, the random self-reducibility of
the discrete logarithm problem a small g is equally secure as any other generator of the same group
*/
pub fn generate_shared_params() -> (BigUint,BigUint) {
    let p = loop {
        let q = prime::new(128).unwrap();
        let safe_prime = BigUint::new([2].to_vec()) * q + BigUint::new([1].to_vec());
        if safe_prime::check(&safe_prime) {
            break safe_prime;
        }
    };
    return (p, BigUint::new([2].to_vec()));
}

/* shared secret is a modular exponentiation (modulus p) of the received public key
 with their own secret key
 */
pub fn generate_shared_secret(p: &BigUint, a_sk: BigUint, b_pk: BigUint) -> BigUint{
    return BigUint::modpow(&b_pk,&a_sk,&p);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn diffie_hellman_shared_secret_matches() {
        // Generate the public parameters prime (p) and generator (g) for both Alice and Bob
        let (prime, generator) = generate_shared_params();
        println!("Prime: {:?} Generator: {:?}", prime, generator);

        // Generate Keypair for Alice and Bob
        let alice = Keypair::generate(&prime, &generator);
        let bob = Keypair::generate(&prime, &generator);

        println!();
        println!("Alice: {:?}", alice);
        println!("Bob: {:?}", bob);

        /*
        Alice computes her shared secret key from the received Bob's public key (b)
         and her private key (alice.sk)
         */
        let alice_shared_secret = generate_shared_secret(&prime, alice.sk, bob.pk);

        /*
        Bob computes his shared secret key from the received Alice's public key (a)
        and her private key (bob.sk)
         */
        let bob_shared_secret = generate_shared_secret(&prime, bob.sk, alice.pk);

        // Verify shared secret keys computed by both parties
        println!();
        println!("Alice's shared secret {:?}", alice_shared_secret.clone());
        println!("Bob's shared secret {:?}", bob_shared_secret.clone());
        assert_eq!(alice_shared_secret, bob_shared_secret);

        println!();
        println!("Done.")
    }
}

# diffie_hellman_key_exchange_test/lib.rs
Use rand = "0.8.5", glass_pumpkin = "1.2.0", num-bigint = { version = "0.4", features = ["rand"] } as dependencies
glass_pumpkin is used for safe primes and num-bigint is used for modular arithmetic.

Key Generation protocol: To generate the public parameters called prime modulus (p) and generator (g)of datatype BigUint which is a member of the module "num_bigint". p is a safe prime such that q = (p-1)/2 is also a prime generated from num_bigint::BigUint. 

Use a generator with a fixed value of 2 and a random safe prime , because the random self-reducibility ofthe discrete logarithm problem a small g is equally secure as any other generator of the same group.

Security Assumptions: 
The number field sieve algorithm solves the discrete logarithmic problem in 4 steps, where the first 3 steps depend on the order of the group G. The Logjam attack used this vulnerability to compromise a variety of Internet services that allowed the use of groups whose order was a 512-bit prime number, so called export grade. The authors needed several thousand CPU cores for a week to precompute data for a single 512-bit prime. Once that was done, individual logarithms could be solved in about a minute using two 18-core Intel Xeon CPUs. It turns out that much Internet traffic uses one of a handful of groups that are of order 1024 bits or less. By precomputing the first three steps of the number field sieve for the most common groups, an attacker need only carry out the last step, which is much less computationally expensive than the first three steps, to obtain a specific logarithm.
To avoid these vulnerabilities, the Logjam authors recommend use of the order, p, of the Diffie–Hellman group should be at least 2048 bits. They estimate that the pre-computation required for a 2048-bit prime is 109 times more difficult than for 1024-bit primes.

Diffie-Hellman Key Exchange protocol with a toy example:
1. Alice and Bob publicly agree to use a modulus p = 23 and generator g = 5 (which is a primitive root modulo 23).

2. Alice chooses a secret integer x = 4, then sends Bob A = g^x mod p
3. 
            A = 5^4 mod 23 = 4
            
3. Bob chooses a secret integer y = 3, then sends Alice B = g^y mod p
4. 
            B = 5^3 mod 23 = 10
            
4. Alice computes a = B^x mod p
5. 
            a = 10^4 mod 23 = 18
            
5. Bob computes b = A^y mod p
6. 
            b = 4^3 mod 23 = 18
            
6. Alice and Bob now share a secret (the number 18).
Both Alice and Bob have arrived at the same values because under mod p,


# dhke_async_channel_oneshot/main.rs

In Oneshot (single producer/single consumer) channel, each channel instance can only transport a single message. Here is an example of how to use oneshot (https://docs.rs/oneshot/latest/oneshot/)

let (sender, receiver) = oneshot::channel();

thread::spawn(move || {
    sender.send("Hello from worker thread!");
});

let message = receiver.recv().expect("Worker thread does not want to talk :(");

println!("A message from a different thread: {}", message);




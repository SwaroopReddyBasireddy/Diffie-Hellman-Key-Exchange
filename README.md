# Diffie-Hellman-Key-Exchange-test/lib.rs
Use rand = "0.8.5", glass_pumpkin = "1.2.0", num-bigint = { version = "0.4", features = ["rand"] } as dependencies
glass_pumpkin is used for safe primes and num-bigint is used for modular arithmetic.

Key Generation protocol: To generate the public parameters called prime modulus (p) and generator (g)of datatype BigUint which is a member of the module "num_bigint". p is a safe prime such the q = (p-1)/2is also a prime generated from num_bigint::BigUint. 

Use a generator with a fixed value of 2 and a random safe prime , because the random self-reducibility ofthe discrete logarithm problem a small g is equally secure as any other generator of the same group.

Security Assumptions: 
The number field sieve algorithm solves the discrete logarithmic problem in 4 steps, where the first 3 steps depend on the order of the group G. The Logjam attack used this vulnerability to compromise a variety of Internet services that allowed the use of groups whose order was a 512-bit prime number, so called export grade. The authors needed several thousand CPU cores for a week to precompute data for a single 512-bit prime. Once that was done, individual logarithms could be solved in about a minute using two 18-core Intel Xeon CPUs. It turns out that much Internet traffic uses one of a handful of groups that are of order 1024 bits or less. By precomputing the first three steps of the number field sieve for the most common groups, an attacker need only carry out the last step, which is much less computationally expensive than the first three steps, to obtain a specific logarithm.

To avoid these vulnerabilities, the Logjam authors recommend use of elliptic curve cryptography, for which no similar attack is known. Failing that, they recommend that the order, p, of the Diffie–Hellman group should be at least 2048 bits. They estimate that the pre-computation required for a 2048-bit prime is 109 times more difficult than for 1024-bit primes.

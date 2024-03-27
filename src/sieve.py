def sieve(up_to: int) -> list:
    primes = list(range(2, up_to))
    prime_index = 0

    while prime_index < len(primes):
        composite = primes[prime_index]
        while composite < up_to:
            composite += primes[prime_index]
            if composite in primes:
                primes.remove(composite)

        prime_index += 1
    return primes

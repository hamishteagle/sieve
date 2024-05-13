def sieve(up_to):
    is_prime = [True] * (up_to + 1)
    primes = []

    for num in range(2, up_to + 1):
        if is_prime[num]:
            primes.append(num)
            multiple = num * 2
            while multiple <= up_to:
                is_prime[multiple] = False
                multiple += num

    return primes

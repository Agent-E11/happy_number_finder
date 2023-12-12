# Happy Number Finder

To determine if a number is "happy" or not:

```py
12 # Pick a number
1^2 + 2^2 = 1 + 4 = 5 # Find the sum of the squares of its digits
5^2 = 25 # Repeat
2^2 + 5^2 = 4 + 25 = 29
2^2 + 9^2 = 4 + 81 = 85
8^2 + 5^2 = 64 + 25 = 89
# etc.
```

If the sequence ends in 1, then the number is "happy". If the sequence gets stuck in any other cycle, then it is "unhappy".

## Performance tests

| Version | Time per 1,000,000 numbers |
| :-----: | -----------------------: |
| 0.1.0   | 24,722,466 μs ≈ 24.7 s   |

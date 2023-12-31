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

| Version | Change                       | Avg. time per 1 mil. numbers | Round 1 | Round 2 | Round 3 | Round 4 | Round 5 |
| :-----: | ---------------------------- | ---------------------------: | ------- | ------- | ------- | ------- | ------- |
| 0.1.0   | Initial                      | 5,902,125 μs ≈ 5.90 s        | 5783038 | 5988484 | 6020976 | 5737952 | 5980174 |
| 0.1.1   | Added binary search          | 6,671,577 μs ≈ 6.67 s        | 6471872 | 6832003 | 6626129 | 6572571 | 6855307 |
| 0.1.2   | Used BTreeSet instead of Vec | 7,042,922 μs ≈ 7.04 s        | 7055865 | 7189209 | 6979918 | 7044653 | 6944967 |

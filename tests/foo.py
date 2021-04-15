import timeit

runs = 100_000
time = timeit.timeit(
    "x = lookup['bob-69']",
    "lookup = {f'bob-{i}': i for i in range(10_000)}",
    number=100_000,
)

total_ms = time * 1000
avg_ms = (total_ms / runs) * (10 ** 6)
print(f"{total_ms}ms total, {avg_ms}ns per call")


runs = 100_000
time = timeit.timeit(
    "x = lookup[69]",
    "lookup = [f'bob-{i}' for i in range(10_000)]",
    number=100_000,
)

total_ms = time * 1000
avg_ms = (total_ms / runs) * (10 ** 6)
print(f"{total_ms}ms total, {avg_ms}ns per call")
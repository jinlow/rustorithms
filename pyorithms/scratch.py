import pyorithmslib as ps
from statistics import stdev, mean
import time
from typing import List
import random

random.seed(123)
rand_ints = [random.choice(range(10_000)) for i in range(300_000)]

RUNS = 300

qsort_time: List[float] = []
for i in range(RUNS):
    start = time.time()
    ps.qsort(rand_ints)
    qsort_time.append(time.time() - start)

sorted_time: List[float] = []
for i in range(RUNS):
    start = time.time()
    sorted(rand_ints)
    sorted_time.append(time.time() - start)

measures = ["Min", "Max", "Mean", "Stdev"]
print("      Function: " + "".join(f"{m:<9}" for m in measures))
print(
    f"ps.quicksort(): {min(qsort_time):.5f}  "
    + f"{max(qsort_time):.5f}  "
    + f"{mean(qsort_time):.5f}  "
    + f"{stdev(qsort_time):.5f}  "
)

print(
    f"sorted()      : {min(sorted_time):.5f}  "
    + f"{max(sorted_time):.5f}  "
    + f"{mean(sorted_time):.5f}  "
    + f"{stdev(sorted_time):.5f}  "
)

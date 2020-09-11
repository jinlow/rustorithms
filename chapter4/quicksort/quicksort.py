from typing import List


def swap(x, i, j):
    x[i], x[j] = x[j], x[i]


def move_front(x, i, j):
    assert j > i, "i must be smaller than j"
    for k in range(j - i):
        swap(x, j - (k + 1), j - k)


def pivot_list(x):
    pivot = 0
    for i in range(1, len(x)):
        if x[i] < x[pivot]:
            move_front(x, pivot, i)
            pivot += 1
    return pivot


def _quicksort1(x):
    if len(x) < 2:
        return x
    else:
        pivot = pivot_list(x)
        return _quicksort1(x[:pivot]) + [x[pivot]] + _quicksort1(x[(pivot + 1) :])


def quicksort1(x: List):
    return _quicksort1(x.copy())


def pivot_list_fl(x):
    pivot = 0
    low = 1
    max_idx = high = len(x) - 1
    while low < high:
        while (x[high] >= x[pivot]) & (high > 0):
            high -= 1
        while (x[low] <= x[pivot]) & (low < max_idx):
            low += 1
        if low < high:
            swap(x, high, low)
    swap(x, pivot, high)
    return high


def _quicksort2(x):
    if len(x) < 2:
        return x
    else:
        pivot = pivot_list_fl(x)
        return _quicksort2(x[:pivot]) + [x[pivot]] + _quicksort2(x[(pivot + 1) :])


def quicksort2(x):
    return _quicksort2(x.copy())


a1 = [3, 1, 4, 5, -1, -6]
print(f"The unsorted list: {a1}")
print(f"The sorted list {quicksort1(a1)}")


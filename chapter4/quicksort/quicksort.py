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


def _quicksort(x):
    if len(x) < 2:
        return x
    else:
        pivot = pivot_list(x)
        return _quicksort(x[:pivot]) + [x[pivot]] + _quicksort(x[(pivot + 1) :])


def quicksort(x: List):
    return _quicksort(x.copy())


a1 = [3, 1, 4, 5, -1, -6]
print(f"The unsorted list: {a1}")
print(f"The sorted list {quicksort(a1)}")


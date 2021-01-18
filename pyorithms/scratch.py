from pyorithms import quicksort as qs

i = [4, 3, 5, 1]
print(f"Int: Unsorted {i}, sorted {qs.qsort(i)}")

s = ["4", "3", "5", "1"]
print(f"Str: Unsorted {i}, sorted {qs.qsort(s)}")

f = [4.0, 3.0, 5.0, 1.0]
print(f"Float: Unsorted {i}, sorted {qs.qsort(f)}")

def quicksort(array):
    if len(array) < 2:
        return array
    else:
        pivot = array[0]
        less = [i for i in array[1:] if i <= pivot]
    
        greater = [i for i in array[1:] if i > pivot]

        return quicksort(less) + [pivot] + quicksort(greater)
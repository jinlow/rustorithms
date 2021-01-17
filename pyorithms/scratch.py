from pyorithms import quicksort as qs

i = [4, 3, 5, 1]
print(f"Int: Unsorted {i}, sorted {qs.qsort_i32(i)}")

s = ["4", "3", "5", "1"]
print(f"Str: Unsorted {i}, sorted {qs.qsort_str(s)}")

f = [4.0, 3.0, 5.0, 1.0]
print(f"Float: Unsorted {i}, sorted {qs.qsort_f32(f)}")


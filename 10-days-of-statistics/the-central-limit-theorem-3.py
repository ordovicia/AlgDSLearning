import math

sample_mean = 500
sample_stddev = 80 / math.sqrt(100)  # = 8
z = 1.96
a = sample_mean - z * sample_stddev
b = sample_mean + z * sample_stddev
print(a)
print(b)

import numpy as np
from numpy.linalg import inv

tmp = [int(d) for d in input().split()]
m = tmp[0]
n = tmp[1]

xs = []
ys = []
for _ in range(n):
    tmp = [float(d) for d in input().split()]
    xs.append([1] + tmp[:-1])
    ys.append(tmp[-1])
xs = np.array(xs)
ys = np.array(ys)

b = xs.T.dot(xs)
b = inv(b)
b = b.dot(xs.T).dot(ys)

q = int(input())
for _ in range(q):
    xs = [1] + [float(x) for x in input().split()]
    xs = np.array(xs)
    print(xs.dot(b))

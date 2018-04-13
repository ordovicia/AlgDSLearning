x = []
y = []

for _ in range(5):
    d = [int(d) for d in input().split()]
    x.append(d[0])
    y.append(d[1])

n = len(x)
sum_x = sum(x)
sum_y = sum(y)

b = n * sum(x * y for (x, y) in zip(x, y)) - sum_x * sum_y
b /= n * sum(x * x for x in x) - sum_x * sum_x

avg_x = sum_x / n
avg_y = sum_y / n
a = avg_y - b * avg_x

# y = a + b x
print("{:.3}".format(a + b * 80, ))

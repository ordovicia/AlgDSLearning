n = int(input())
x = [float(x) for x in input().split()]
y = [float(y) for y in input().split()]

avg_x = sum(x) / n
avg_y = sum(y) / n

delta_x = [x - avg_x for x in x]
delta_y = [y - avg_y for y in y]

stdev_x = sum(x * x for x in delta_x) ** 0.5
stdev_y = sum(y * y for y in delta_y) ** 0.5

coeff = sum(x * y for (x, y) in zip(delta_x, delta_y)) / (stdev_x * stdev_y)
print("{:.3f}".format(coeff, ))

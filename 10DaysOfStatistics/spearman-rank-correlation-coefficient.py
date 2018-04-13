n = int(input())
x = [float(x) for x in input().split()]
y = [float(y) for y in input().split()]

x_sorted = sorted(x)
y_sorted = sorted(y)


def diff(idx):
    def rank(x, data):
        return data.index(x) + 1

    return rank(x[idx], x_sorted) - rank(y[idx], y_sorted)


d = [diff(i) for i in range(n)]
coeff = 1 - 6 * sum(d * d for d in d) / (n * (n * n - 1))
print("{:.3f}".format(coeff, ))

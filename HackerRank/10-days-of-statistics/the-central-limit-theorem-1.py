import math

total_mean = 49 * 205
total_stddev = 7 * 15
prob = (1 + math.erf((9800 - total_mean) / (math.sqrt(2) * total_stddev))) / 2
print("{}".format(round(prob, 4), ))

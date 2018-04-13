import math

want_mean = 100 * 2.4
want_stddev = 10 * 2.0
available = 250
prob = (1 + math.erf((available - want_mean) / (math.sqrt(2) * want_stddev))) / 2
print("{}".format(round(prob, 4), ))

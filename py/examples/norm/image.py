from numpy import *
from pylab import *

r1, number1 = genfromtxt("../../../data/examples/out/norm/norm1.dat").T
r2, number2 = genfromtxt("../../../data/examples/out/norm/norm2.dat").T

plt.plot(r1[0:-1], number1[0:-1], 'r--', r2[0:-1], number2[0:-1], 'b--')
plt.show()
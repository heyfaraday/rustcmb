from numpy import *
from pylab import *

N = 64 + 1
M = 64 + 1

# xlim([0,1])
# ylim([0,1])

x, y, z = genfromtxt("../../../data/examples/out/fft_2d/f.dat").T
x = x.reshape(N, M)
y = y.reshape(N, M)
z = z.reshape(N, M)
pcolor(x, y, z)

show()
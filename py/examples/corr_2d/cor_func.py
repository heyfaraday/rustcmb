from numpy import *
from pylab import *

size = 64

plt.figure(figsize=(8, 8))
x, y, z = genfromtxt("../../../../data/out/rust-examples/corr_2d/cor_func.dat").T
x = x.reshape(2 * size - 1, 2 * size - 1)
y = y.reshape(2 * size - 1, 2 * size - 1)
z = z.reshape(2 * size - 1, 2 * size - 1)
pcolor(x, y, z)
show()

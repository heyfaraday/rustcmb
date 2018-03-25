from numpy import *
from pylab import *

size = 64

plt.figure(figsize=(8, 8))
x, y, z = genfromtxt("../../../../data/out/rust-examples/corr_2d/field.dat").T
x = x.reshape(size + 1, size + 1)
y = y.reshape(size + 1, size + 1)
z = z.reshape(size + 1, size + 1)
pcolor(x, y, z)
show()

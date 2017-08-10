from numpy import *
from pylab import *

size = 8
plt.figure(figsize=(8, 8))
x, y, z = genfromtxt(
    "../../../data/examples/out/fft_2d/first_realization_field.dat").T
x = x.reshape(size + 1, size + 1)
y = y.reshape(size + 1, size + 1)
z = z.reshape(size + 1, size + 1)
pcolor(x, y, z)
show()

size = 8
plt.figure(figsize=(8, 8))
x, y, z = genfromtxt(
    "../../../data/examples/out/fft_2d/second_realization_field.dat").T
x = x.reshape(size + 1, size + 1)
y = y.reshape(size + 1, size + 1)
z = z.reshape(size + 1, size + 1)
pcolor(x, y, z)
show()

size = 8
plt.figure(figsize=(8, 8))
x, y, z = genfromtxt(
    "../../../data/examples/out/fft_2d/first_realization_field_from_back.dat").T
x = x.reshape(size + 1, size + 1)
y = y.reshape(size + 1, size + 1)
z = z.reshape(size + 1, size + 1)
pcolor(x, y, z)
show()

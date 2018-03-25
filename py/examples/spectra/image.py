from numpy import *
from pylab import *

size = 512

plt.figure(figsize=(8, 8))
x, y, z = genfromtxt("../../../../data/out/rust-examples/spectra/gasdev_field.dat").T
x = x.reshape(size + 1, size + 1)
y = y.reshape(size + 1, size + 1)
z = z.reshape(size + 1, size + 1)
pcolor(x, y, z)
show()

plt.figure(figsize=(8, 8))
x, y, z = genfromtxt(
    "../../../../data/out/rust-examples/spectra/gasdev_exp_k0_field.dat").T
x = x.reshape(size + 1, size + 1)
y = y.reshape(size + 1, size + 1)
z = z.reshape(size + 1, size + 1)
pcolor(x, y, z)
show()

plt.figure(figsize=(8, 8))
x, y, z = genfromtxt(
    "../../../../data/out/rust-examples/spectra/gasdev_max_k_field.dat").T
x = x.reshape(size + 1, size + 1)
y = y.reshape(size + 1, size + 1)
z = z.reshape(size + 1, size + 1)
pcolor(x, y, z)
show()

plt.figure(figsize=(8, 8))
x, y, z = genfromtxt(
    "../../../../data/out/rust-examples/spectra/gasdev_exp_and_sin_field.dat").T
x = x.reshape(size + 1, size + 1)
y = y.reshape(size + 1, size + 1)
z = z.reshape(size + 1, size + 1)
pcolor(x, y, z)
show()

r1, number1 = genfromtxt(
    "../../../../data/out/rust-examples/spectra/returned_exp_k0_spectra.dat").T

plot(r1[0:-1], number1[0:-1], 'r--')
show()

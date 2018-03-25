from numpy import *
from pylab import *

size = 64

plt.figure(figsize=(8, 8))
x, y, z = genfromtxt(
    "../../../../data/out/rust-examples/scalar_field/empty_field.dat").T
x = x.reshape(size + 1, size + 1)
y = y.reshape(size + 1, size + 1)
z = z.reshape(size + 1, size + 1)
pcolor(x, y, z)
show()

r1, number1 = genfromtxt(
    "../../../../data/out/rust-examples/scalar_field/spectra.dat").T

plot(r1[0:-1], number1[0:-1], 'r--')
show()

r1, number1 = genfromtxt(
    "../../../../data/out/rust-examples/scalar_field/norm_for_spectra.dat").T

plot(r1[0:-1], number1[0:-1], 'r--')
show()

r1, number1 = genfromtxt(
    "../../../../data/out/rust-examples/scalar_field/dispersion.dat").T

plot(r1[0:-1], number1[0:-1], 'r--')
show()

r1, number1 = genfromtxt(
    "../../../../data/out/rust-examples/scalar_field/correlation_function.dat").T

plot(r1[0:-1], number1[0:-1], 'r--')
show()

r1, number1 = genfromtxt(
    "../../../../data/out/rust-examples/scalar_field/norm_for_correlation_function.dat"
).T

plot(r1[0:-1], number1[0:-1], 'r--')
show()

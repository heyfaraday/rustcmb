from numpy import *
from pylab import *

plt.figure(figsize=(10, 8))
phi_1, phi_2, v1, v2 = genfromtxt(
    '../../../../data/out/rust-examples/mcmc/sample.dat').T

plt.hist2d(phi_1, phi_2, bins=100)
plt.colorbar()
plt.show()

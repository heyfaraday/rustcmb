from numpy import *
from pylab import *

plt.figure(figsize=(10, 8))
phi_1, phi_2, v1, v2 = genfromtxt(
    '../../../../data/out/rust-examples/mcmc/sample.dat').T

n, bins, patches = plt.hist(
    phi_1 + phi_2, 200, normed=1, facecolor='green', alpha=0.75)
plt.show()

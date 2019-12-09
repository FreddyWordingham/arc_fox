#!/usr/bin/env python
import os
import sys
import matplotlib
import matplotlib.pyplot as plt
import numpy as np
import math
import scipy.signal as sig
from scipy.optimize import curve_fit
from pylab import meshgrid, cm, imshow, contour, clabel, colorbar, axis, title, show


#directory = "/Users/lm579/Documents/Lab_231019/Map_4pc/"
tot_raman = []
tot_skip = []

f = open("/Users/lm579/Projects/arc/output/ptfe/tot_raman_100k_martha.txt", 'r')
for line in f:
    data = line.split(",")
    tot_raman.append(np.float(data[0]))
    #tot_skip.append(np.float(data[1]))

plt.plot(tot_raman, label = "Total Raman signal")
#plt.scatter(range(len(tot_skip)), tot_skip, label = "Total skipped photons")
#plt.legend()
plt.show()

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

tot_raman = []
tot_detected = []
#loc = [x for x in range(-0.01,0.01, 0.001)]
loc = np.arange(-0.01, 0.011, 0.001).tolist()

f = open("/Users/lm579/Projects/arc/output/ptfe/Ramans_1e8phot_quarterpc.txt", 'r')
for line in f:
    data = line.split(",")
    tot_raman.append(np.float(data[0]))
    tot_detected.append(np.float(data[1]))


div_max = max(tot_detected)
for i in range(len(tot_detected)):
    tot_detected[i] = tot_detected[i]/div_max
#plt.plot(loc[:len(tot_raman)], tot_raman, label = "Total Raman photons made", marker = "x")
#plt.plot(loc[:len(tot_detected)], tot_detected, label = "Total Raman photons detected", marker = "x")
#plt.legend()
#plt.show()

z,(ax1, ax2) = plt.subplots(2,1, sharex=True)
ax1.plot(loc[:len(tot_raman)], tot_raman, marker = "x")
ax2.plot(loc[:len(tot_detected)], tot_detected, marker = "x")
ax2.set_ylim(0, 1.5)
plt.show()

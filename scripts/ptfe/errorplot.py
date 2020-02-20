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
sum_raman = []
sum_detected = []
plot_raman = []
plot_detected = []
mean_raman = []
mean_detected = []
var_raman = []
var_detected = []
#loc = [x for x in range(-0.01,0.01, 0.001)]
loc = np.arange(-0.01, 0.011, 0.001).tolist()

f = open("/Users/lm579/Projects/arc/output/ptfe/1pc_100x_1e-4raman.txt", 'r')
for line in f:
    data = line.split(",")
    tot_raman.append(np.float(data[0]))
    tot_detected.append(np.float(data[1]))

for i in range(0,len(tot_raman), 10):
    sum_raman.append(tot_raman[i:i+10])
    sum_detected.append(tot_detected[i:i+10])

for i in range(len(sum_raman)):
    plot_raman.append(np.sum(sum_raman[i]))
    plot_detected.append(np.sum(sum_detected[i]))
    mean_raman.append(np.sum(sum_raman[i])/len(sum_raman[i]))
    mean_detected.append(np.sum(sum_detected[i])/len(sum_detected[i]))

for i in range(len(sum_raman)):
    var_raman.append((np.sum((sum_raman[i] - mean_raman[i])**2)/len(sum_raman[i]))/np.sqrt(10000000))
    var_detected.append(np.sum((sum_detected[i] - mean_detected[i])**2)/len(sum_detected[i]))

#print max(plot_detected)

max_div = max(plot_detected)
#plt.plot(loc[:len(tot_raman)], tot_raman, label = "Total Raman photons made", marker = "x")
#plt.plot(loc[:len(tot_detected)], tot_detected, label = "Total Raman photons detected", marker = "x")
#plt.legend()
#plt.show()

z,(ax1, ax2) = plt.subplots(2,1, sharex=True)
ax1.errorbar(loc[:len(plot_raman)], plot_raman, xerr = None, yerr = var_raman, marker = "x")
#ax2.plot(loc[:len(plot_detected)], plot_detected/max_div, marker = "x")
ax2.errorbar(loc[:len(plot_detected)], plot_detected/max_div, xerr=None, yerr=var_detected/max_div, marker = "x")
plt.show()

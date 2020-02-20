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
median_raman = []
median_detected = []

#location for MC in mm to match the experimental plot
loc = np.arange(3, 24, 1.0).tolist()

#EXPERIMENTAL DATA LOADING AND ANALYSIS
###############################################################################
directory = "/Users/lm579/Projects/arc/output/ptfe/experimental/Map_1pc/"

split_data = []
shift = []
intensities = []
i = 0
j = 0
for file in sorted(os.listdir(directory)):
    if file.endswith(".asc"):
        f = open(directory+file,'r')
        shift.append([])
        intensities.append([])
        for line in f:
            raw_data = line.split(",")
            shift[i].append(float(raw_data[0]))
            raw_intensities = raw_data[2:len(raw_data)-1]
            raw_intensities = [int(x) for x in raw_intensities]
            intensities[i].append(raw_intensities)
            j += 1
        f.close()
        i += 1

median = [[] for x in intensities]
for i in range(len(intensities)):
    for j in range(len(intensities[0])):
        median[i].append([])
        median[i][j] = np.median(intensities[i][j])

full_plot = []
full_median = []
plot_shift = []
plot_median = []
beforebin_shift = []
beforebin_median = []
afterbin_shift = []
afterbin_median = []

for i in range(len(intensities)):
    full_plot.append([])
    full_median.append([])
    plot_shift.append([])
    plot_median.append([])
    beforebin_shift.append([])
    beforebin_median.append([])
    afterbin_shift.append([])
    afterbin_median.append([])
    for j in range(len(shift[i])):
        full_plot[i].append(shift[i][j])
        full_median[i].append(median[i][j])
        if shift[i][j] > 710.0 and shift[i][j] < 760.0:
            plot_shift[i].append(shift[i][j])
            plot_median[i].append(median[i][j])
        if shift[i][j] > 660.0 and shift[i][j] < 710.0:
            beforebin_shift[i].append(shift[i][j])
            beforebin_median[i].append(median[i][j])
        if shift[i][j] > 760.0 and shift[i][j] < 800.0:
            afterbin_shift[i].append(shift[i][j])
            afterbin_median[i].append(median[i][j])

beforebin_avg = []
afterbin_avg = []
slope = []
const = []

for i in range(len(beforebin_median)):
    beforebin_avg.append(sum(beforebin_median[i])/len(beforebin_median[i]))
    afterbin_avg.append(sum(afterbin_median[i])/len(afterbin_median[i]))
    beforeshift_avg= sum(beforebin_shift[i])/len(beforebin_shift[i])
    aftershift_avg = sum(afterbin_shift[i])/len(afterbin_shift[i])
    slope.append((afterbin_avg[i]-beforebin_avg[i])/(aftershift_avg - beforeshift_avg))
    const.append(beforebin_median[i][0]-(slope[i]*beforebin_shift[i][0]))

adj_plot_median = []
for i in range(len(intensities)):
    adj_plot_median.append([])
    for j in range(len(plot_shift[i])):
        adj_plot_median[i].append(plot_median[i][j] - ((slope[i]*plot_shift[i][j])+const[i]))

colour = ['red', 'purple', 'orange', 'pink']
plot_shift = np.asarray(plot_shift)
adj_plot_median = np.asarray(adj_plot_median)

labels = ["PTFE at laser", "PTFE at centre", "PTFE at detector"]
opt_params = []
opt_error = []
j=0

for i in range(len(plot_shift)):
    n = len(plot_shift[i])
    mean = sum(plot_shift[i]*adj_plot_median[i])/sum(adj_plot_median[i])
    sigma = sum(adj_plot_median[i]*(plot_shift[i]-mean)**2)/sum(adj_plot_median[i])

    def gaus(x,a,x0,sigma):
     return a*np.exp(-(x-x0)**2/(2*sigma**2))

    popt,pcov = curve_fit(gaus,plot_shift[i],adj_plot_median[i],p0=[max(adj_plot_median[i]),mean,sigma])
    opt_params.append(popt)
    perr = np.sqrt(np.diag(pcov))
    opt_error.append(perr)

opt_a = []
opt_x0 = []
opt_sigma = []

for i in range(len(opt_params)):
    opt_a.append(opt_params[i][0])
    opt_x0.append(opt_params[i][1])
    opt_sigma.append(opt_params[i][2])

area1 = []
area2 = []
calc_int = []
peaks = []
err_calc_int = []
for i in range(len(plot_shift)):
    area1.append(np.trapz(adj_plot_median[i], plot_shift[i]))
    area2.append(np.trapz(adj_plot_median[i]))
    calc_int.append(np.sqrt(2*math.pi)*opt_a[i]*abs(opt_sigma[i]))
    peaks.append(max(adj_plot_median[i]))
    err_calc_int.append(np.sqrt(math.pi*(opt_a[i]*opt_error[i][2])**2 + (opt_sigma[i]*opt_error[i][0])**2))

plot_div = np.average(calc_int)

#MC DATA LOADING AND ANALYSIS
###############################################################################
#f = open("/Users/lm579/Projects/arc/output/ptfe/ramans_1e8phot_quarterpc.txt", 'r')
f = open("/Users/lm579/Projects/arc/output/ptfe/1pc_100x_1e-3raman.txt", "r")
old_file_position = f.tell()
f.seek(0,2)
size = f.tell()
f.seek(old_file_position,0)

for line in f:
    data = line.split(",")
    tot_raman.append(np.float(data[0]))
    tot_detected.append(np.float(data[1]))

if (size > 1000.0):

    for i in range(0,len(tot_raman), 10):
        sum_raman.append(tot_raman[i:i+10])
        sum_detected.append(tot_detected[i:i+10])

    for i in range(len(sum_raman)):
        plot_raman.append(np.sum(sum_raman[i]))
        plot_detected.append(np.sum(sum_detected[i]))
        mean_raman.append(np.average(sum_raman[i]))
        mean_detected.append(np.average(sum_detected[i]))
        median_raman.append(np.median(sum_raman[i]))
        median_detected.append(np.median(sum_detected[i]))

    for i in range(len(sum_raman)):
        var_raman.append(np.sum((sum_raman[i] - mean_raman[i])**2)/len(sum_raman[i]))
        var_detected.append(np.sum((sum_detected[i] - mean_detected[i])**2)/len(sum_detected[i]))

    max_div = np.average(mean_detected)
    plt.errorbar(loc[:len(mean_detected)], mean_detected/max_div, xerr=None, yerr=var_detected/max_div, marker='x', label='Monte Carlo simulation', linestyle='--')

else:
    max_div = np.float64(max(tot_detected))
    plt.errorbar(loc[:len(tot_detected)], tot_detected/max_div, xerr=None, yerr=var_detected/max_div, marker='x', label='Monte Carlo simulation', linestyle='--')

#z,(ax1, ax2) = plt.subplots(2,1, sharex=True)
#ax1.errorbar(loc[:len(plot_raman)], plot_raman, xerr = None, yerr = var_raman, marker = "x")
#ax2.errorbar(loc[:len(plot_detected)], plot_detected/max_div, xerr=None, yerr=var_detected/max_div, marker = "x")

plt.errorbar(range(2,25), calc_int/plot_div, xerr=None, yerr=err_calc_int/plot_div, marker='o',label="Experimental intensity")
#plt.plot(range(2,25), calc_int/plot_div, marker='o', label='Experimental intensity')
plt.xlabel("Depth of PTFE slab")
plt.ylabel("Intensity")
plt.xlim(0, 26)
plt.ylim(0, )
plt.legend()
plt.show()
plt.show()

import matplotlib.pyplot as plt
#slab position in tank
x = [-0.01,-0.009, -0.008, -0.007, -0.006, -0.005, -0.004, -0.003, -0.002, -0.001, 0.0, 0.001, 0.002, 0.003, 0.004, 0.005, 0.006, 0.007, 0.008, 0.009, 0.01]
#buggyboi output 1 million photons, intra same for all wavelengths
y1 = [69.1, 66.0, 71.2, 84.0, 89.3, 75.1, 86.2, 76.6, 65.3, 75.0, 91.0, 71.2, 47.8, 47.7, 49.4, 39.2, 32.7, 24.7, 16.6, 12.9, 8.42]
#TRASH buggyboi output 1 million photons, wavelength dependent intra optical properties, including detector hits, plus speed up
#y2 = [472.8, 525.0, 529.6, 568.4, 586.7, 633.9, 575.6, 620.6, 606.0, 620.3, 610.7, 646.0, 588.3, 617.0, 593.7, 577.3, 550.3, 534.8, 533.0, 506.7, 544.3]
#buggyboi output 1 million photons, wavelength dependent intra optical properties, including detector hits, plus speed up
y3 = [195.9, 201.3, 207.1, 248.4, 254.8, 222.7, 243.9, 236.2, 234.2, 238.5, 217.5, 202.9, 193.9, 193.6, 159.5, 129.4, 96.8, 89.7, 69.0, 41.6, 23.8]


plt.plot(range(len(y3)),y3)
plt.show()

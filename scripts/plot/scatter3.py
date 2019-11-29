#! /usr/bin/env python3

# If you have permission denied error, give yourself permission to run the script.
# sudo chmod 755 '$ARC_DIR/scripts/plot/scatter3.py'

from mpl_toolkits.mplot3d import Axes3D

import csv
import matplotlib.pyplot as plt

with open('./output/hit.csv', newline='\n') as csv_file:
    csv_data = csv.reader(csv_file, delimiter=',')

    xs = []
    ys = []
    zs = []

    for row in csv_data:
        xs.append(float(row[0]))
        ys.append(float(row[1]))
        zs.append(float(row[2]))

print(len(xs), " total points.")

fig = plt.figure()
ax = fig.add_subplot(111, projection='3d')

ax.scatter(xs, ys, zs, c='r', marker='o')

ax.set_xlabel('X Label')
ax.set_ylabel('Y Label')
ax.set_zlabel('Z Label')

plt.show()

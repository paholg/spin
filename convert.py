#!/usr/bin/env python3

import sys
import numpy
import scipy
import scipy.ndimage

nleds = 16
nphi = 72

dphi = 2*numpy.pi / nphi

def main():
    im = scipy.ndimage.imread(sys.argv[1], mode='RGB')

    print(im.shape)
    width = im.shape[0]
    height = im.shape[1]

    cx = int(width/2)
    cy = int(height/2)

    radius = min(cx, cy)

    result = numpy.zeros((nphi, nleds, 3))


    im2 = numpy.zeros_like(im)
    im2[0:cx, 0:cy, :] = im[cx:width, cy:height, :]
    im2[cx:width, 0:cy, :] = im[0:cx, cy:height, :]

    im2[0:cx, cy:height, :] = im[cx:width, 0:cy, :]
    im2[cx:width, cy:height, :] = im[0:cx, 0:cy, :]

    for (r_i, r) in enumerate(numpy.linspace(0.0, radius, nleds)):
        for (phi_i, phi) in enumerate(numpy.arange(2*numpy.pi, 0.0, -dphi)):
            x = int(round(r*numpy.sin(phi)))
            y = int(round(r*numpy.cos(phi)))
            result[phi_i, r_i, 0] = im2[x, y, 0]
            result[phi_i, r_i, 1] = im2[x, y, 1]
            result[phi_i, r_i, 2] = im2[x, y, 2]


    f = open('examples/picture.dat', 'w')
    f.write("""&[
""")
    for (i, strip) in enumerate(result):
        f.write('({},['.format(i*dphi))
        for color in strip:
            f.write('Rgb::new({},{},{}),'.format(int(color[0]), int(color[1]), int(color[2])))
        f.write(']),\n')

    f.write("""];

""")

if __name__ == '__main__':
    main()

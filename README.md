# Sphere-Points
### Inputs
 - Radius range in arbitrary units.
 - Radius resolution divide range equally.
 - Longitudinal range in radians.
 - Longitudinal resolution divide range equally.
 - Latitude range in radians.
 - Latitude resolution divide range equally.

### Outputs
 - Array of cartesian coordinates.

### Fonctions
- Calculate the number of output coordinates for some inputs.
- Calculate the coordinates for some inputs.

### How does it works?
The first point is always at origin. (0.0, 0.0, 0.0)

In 1D, the subsequent points are on a line from origin to "radius range" at interval "radius resolution".

In 2D, the points form a circular sector with an angle equal to "longitude range" divided in multiple 1D lines.
If "longitude range" is 360 the result is in fact a circle.

In 3D, the points form a spherical sector with an angle equal to "latitude range" divided in multiple 1D lines.
If "longitude range" and "latitude range" are 360 the result is in fact a sphere.

Try to visualize a light source with an horizontal and vertical angle and a range.
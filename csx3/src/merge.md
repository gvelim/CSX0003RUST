# Merge Algorithms
## Merging two or more ordered sets
In its simplest form that basic use case is the need to merge two ordered arrays into a single but yet ordered array.
Altough simple, it becomes far more complicated when you consider 
* very large datasets spanning processing nodes (segmentation, map/reduce)
* memory and cpu constraints of embedded systems (in-place, out-of-place) 

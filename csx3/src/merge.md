# Merge Algorithms
## Merging two or more ordered sets
The basic use case for these algorithms is merging two ordered arrays into a single and ordered array.
Although simple, it becomes far more complicated when you consider 
* Very large datasets spanning many processing nodes (segmentation, map/reduce, etc)
* Memory and cpu constraints on embedded systems (in-place, out-of-place) 

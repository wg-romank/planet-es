# Planetes

Procedural planet generation using simplex noise.

Stuff to do:

M0: Landscape
- Mercator texture mapping
- Less confusing landscage generation (remove layers, add 3 types of noise: ridge, simplex, white)
- Ocean floor
- Normal/triplanar mapping

Ocean
- Ocean shader

Athmosphere
- Cloud shader

Presentation mode, controls:
- Touch pan -> rotate
- Touch pinch -> zoom
- Touch pan with 2 fingers -> move

M1: Misc
- integration (issuing/signing)

Optimizations:
- Better mesh generation (more fine-grained control over num polys with ico-sphere sub-divisions)
- Less serde passes when checking parameters


Note: depends on forked version of `icosahedron` crate, based on `vek` instead of `cgmath`. Be sure to clone it alongside this project to build. (https://github.com/wg-romank/icosahedron)

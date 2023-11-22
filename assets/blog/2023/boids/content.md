
# Boids & Quadtrees

Quadtrees are data structures for holding spatial information with a complexity of log(N) for collision detection. These are the foundation of every physics engine. They are also great for boid simulations. Here is an example running in Wasm.


This application is written in rust, using bevy and egui. You can find the source code on my [Github](https://github.com/Lommix/quadtree_boid_simulation)
Here is a picture explaining how the tree maps to 2D Space


![test](/media/boids/quad.jpg)

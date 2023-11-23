# Boids & Quadtrees

Quadtrees are data structures for holding spatial information with a complexity of log(N) for collision detection. These are the foundation of every physics engine. They are also great for boid simulations. Here is an example running in Wasm.


<bevy-runner wasm-path="/media/QuadtreesInRust/boids-quadtree_bg.wasm.gz" canvas-id="boid-canvas" script-path="/media/QuadtreesInRust/boids-quadtree.js" height="800" style="background-color:black;">
    <img style="margin: 0 auto;display:block;" src="/media/QuadtreesInRust/cover.jpg"/>
</bevy-runner>


This application is written in rust, using bevy and egui. You can find the source code on my [Github](https://github.com/Lommix/quadtree_boid_simulation)

Here is a picture explaining how the tree maps to 2D Space

![diagram](/media/QuadtreesInRust/quad.jpg)


Each node, in addition to its spatial information, can be one of two types: a Leaf, which is a data container for object references, or a Branch, which can only hold other nodes.


In rust, we can represent such a type with an enum!

```rust
enum Node{
    Branch(Box<[Node;4]>),
    Leaf(Vec<BodyId>),
}
```

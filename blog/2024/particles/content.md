# Particles Everywhere - Bevy Enoki ([Github](https://github.com/Lommix/bevy_enoki))

In my recent journey as a game developer, I encountered a requirement for a particle system in one of my projects. While Bevy
Hanabi is an excellent and efficient solution for this purpose, there were some challenges. Specifically, Hanabi relies heavily
on compute shaders for particle computations. However, compute shaders are not universally supported, particularly in web and
mobile environments.
Seeking a particle system with CPU-based particle computation but GPU instancing for optimal performance, I decided to develop
my own solution. Not only would this cater to my current project's needs, but it also offered an opportunity to create user-friendly
tools that simplify the creative process.

### The Enoki web editor

<wasm-frame src="https://lommix.com/wasm/particle/index.html" cover="/media/bevy-enoki/cover.jpg">
</wasm-frame>

[Link to the fullscreen Version](https://lommix.com/wasm/particle/index.html)

One distinctive advantage that sets this crate apart from others is the capability to create custom particle
materials. While some crates allow binding textures for particles, this functionality is very basic.
Instead, I seek a more robust solution: the ability to define and utilize custom Fragment Shaders for particles.
This feature goes beyond merely attaching sprites or animations; it empowers users to create sophisticated
particle effects that would be difficult or impossible with simple texture binding alone.

![gif](/media/bevy-enoki/output.gif)

## [Checkout Bevy Enoki on Github](https://github.com/Lommix/bevy_enoki)

If you are looking for full particle dev experience, look no further and start
creating amazing effects in your own game.

//! Home hero — TypeScript client component (Three.js WebGPU particles).

use resuma::prelude::*;

/// Particle field background for the landing hero.
pub fn hero_particles_mount() -> View {
    client_component(
        ClientComponent::new("hero-particles")
            .class("hero-particles")
            .lazy(true),
    )
}

# Powder Game - Bevy Edition

Inspired by the original [Powder Game](https://dan-ball.jp/en/javagame/dust/),
 [The Powder Toy](https://powdertoy.co.uk/), and
[Sandspiel](https://sandspiel.club/), this project aims to recreate the game
using the Bevy game engine. The goal is to provide a similar experience with
enhanced features and performance. It is a learning project for Bevy and Rust,
focusing on game development, physics simulation, and user interaction.

This game is created as part of [Bevy Jam 6](https://itch.io/jam/bevy-jam-6).

## 1. Components & Resources

```mermaid
classDiagram
    %% Core Components
    class Position {
      +x : f32
      +y : f32
    }
    class Velocity {
      +vx : f32
      +vy : f32
    }
    class ParticleType {
      Sand
      Water
      Oil
      Fire
      Steam
      Stone
    }
    class Density {
      +value : f32
    }
    class Temperature {
      +deg_c : f32
    }
    class Flammable {
    }
    class Lifetime {
      +t : f32
    }

    %% Data-Driven Rules
    class InteractionRules {
      +rules : Map<(ParticleType,ParticleType),Effect>
    }
    class Effect {
      Swap
      Merge
      Spawn
      ChangeType
    }

    %% Systems
    class MovementSystem
    class BuoyancySystem
    class CombustionSystem
    class PhaseChangeSystem
    class LifetimeSystem
    class RenderingSystem

    %% Relations
    MovementSystem      --> BuoyancySystem
    BuoyancySystem      --> CombustionSystem
    CombustionSystem    --> PhaseChangeSystem
    PhaseChangeSystem   --> LifetimeSystem
    LifetimeSystem      --> RenderingSystem

    Density            --> BuoyancySystem
    Velocity           --> MovementSystem
    ParticleType       --> CombustionSystem
    ParticleType       --> PhaseChangeSystem
    Temperature        --> PhaseChangeSystem
    Flammable          --> CombustionSystem
    InteractionRules   --> BuoyancySystem
    InteractionRules   --> CombustionSystem
    InteractionRules   --> PhaseChangeSystem
````

* **Position, Velocity**: spatial data per particle
* **ParticleType**: defines material kinds
* **Density, Temperature, Flammable**: trait components for specialized physics
* **Lifetime**: optional TTL for short-lived effects
* **InteractionRules** + **Effect**: data-driven mapping of “when X meets Y → do Z”

---

## 2. System Pipeline

```mermaid
flowchart LR
  subgraph Update
    M[MovementSystem]
    B[BuoyancySystem]
    C[CombustionSystem]
    P[PhaseChangeSystem]
    L[LifetimeSystem]
    M --> B --> C --> P --> L
  end
  subgraph Render
    L --> R[RenderingSystem]
  end
```

1. **MovementSystem**

   * Apply velocity, gravity.
2. **BuoyancySystem**

   * Swap light vs. heavy liquids (oil on water).
3. **CombustionSystem**

   * Burn flammables on fire contact.
4. **PhaseChangeSystem**

   * Convert water → steam on heat.
5. **LifetimeSystem**

   * Despawn expired/consumed entities.
6. **RenderingSystem**

   * Sync sprites/transforms and draw.

---

## 3. Example Rules

```rust
let mut rules = InteractionRules::default();

// Oil floats on water
rules.insert(
    (ParticleType::Oil, ParticleType::Water),
    Effect::Swap,
);

// Oil burns to fire
rules.insert(
    (ParticleType::Oil, ParticleType::Fire),
    Effect::ChangeType(ParticleType::Fire),
);

// Water → Steam on fire
rules.insert(
    (ParticleType::Water, ParticleType::Fire),
    Effect::ChangeType(ParticleType::Steam),
);
```

---

## 4. Four-Hour Priorities

1. **Grid & Movement**: build `WorldGrid`, implement `MovementSystem`.
2. **Single Particle**: get “sand” falling correctly.
3. **First Interaction**: add oil/water buoyancy.
4. **Layer Effects**: add combustion and steam rules.
5. **Rendering**: minimal sprite or colored quad per particle.

> Keep systems small, data-driven, and pipeline-ordered. Iterate one rule at a time.

# tank_game
# tank_game: AI Competition Game Engine

This repository contains **tank_game**, a customizable framework designed for an AI competition where players program and compete using autonomous tanks. The goal of the game is simple: **the last tank standing wins**.

### Key Features:
- **REST API Integration**: Players control their tanks by developing a REST API that interacts with the game engine. Each tank is driven by the AI logic provided via the API, allowing full control over movement, turret rotation, and shooting.
  
- **AI Flexibility**: The AI controlling each tank can range from a simple decision tree using basic conditional logic (if-else statements) to a sophisticated machine learning model for advanced strategies.
  
- **Survival Objective**: The game operates as a survival challenge where players compete to keep their tank alive while eliminating opponents. The last surviving tank is declared the winner.

- **Optional Player Mode**: Players can opt to play against their own AI-driven tank to test and refine strategies.

This game engine serves as the foundation for participants to develop, test, and refine their AI models in a competitive yet accessible environment.

# Args

# Todo

# Game Engine To-Do List

## Rendering
- [X] **Optional Rendering**  
  - Implement **headless mode** for the game engine, to run without rendering.
  - Set up **default rendering** mode with visual outputs.

## User Input Processing
- [-] **Movement**  
  - Implement player-controlled movement inputs for tanks.
- [X] **Turret Rotation**  
  - Add controls to rotate turrets based on user input.
- [-] **Shooting**  
  - Allow user-triggered shooting action via inputs.

## Sensor Inputs
- [X] **Hull Ray Casts**  
  - Implement hull ray casting for detecting objects.
- [X] **Turret Hull Ray Casts**  
  - Set up ray casts originating from the turret to detect enemies.

## Shooting Mechanism
- [ ] **Bullet Generation**  
  - Create bullet instances when the player shoots.
- [ ] **Bullet Destruction**  
  - Destroy bullet on impact with tank or wall.
- [ ] **Tank Destruction**  
  - Destroy enemy tanks upon being hit by a bullet.
- [ ] **End Game Logic**  
  - Update game state when a tank is destroyed and determine the winner.

## Collision Handling
- [ ] **Wall and Tank Collisions**  
  - Implement collision detection to prevent tanks from moving through walls and other tanks.

## Player Inputs
- [x] **Keyboard Inputs**  
  - Set up controls using keyboard input to move and interact.
- [ ] **REST API Inputs**  
  - Integrate support for controlling the game using REST API inputs.

## Sounds (SECONDARY)
- [ ] **Shooting Sound Effect**  
  - Play sound effect when shooting.
- [ ] **Explosion Sound Effect**  
  - Trigger explosion sound when a tank or bullet is destroyed.

## Integration Tests
- [ ] **Headless Mode Test**  
  - Verify the game engine runs correctly in headless mode.
- [ ] **Rendering Mode Test**  
  - Test if the game works correctly in rendering mode.
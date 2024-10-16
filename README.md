# tank_game
# tank_game: AI Competition Game Engine

This repository contains **tank_game**, a customizable framework designed for an AI competition where players program and compete using autonomous tanks. The goal of the game is simple: **the last tank standing wins**.

### Key Features:
- **REST API Integration**: Players control their tanks by developing a REST API that interacts with the game engine. Each tank is driven by the AI logic provided via the API, allowing full control over movement, turret rotation, and shooting.
  
- **AI Flexibility**: The AI controlling each tank can range from a simple decision tree using basic conditional logic (if-else statements) to a sophisticated machine learning model for advanced strategies.
  
- **Survival Objective**: The game operates as a survival challenge where players compete to keep their tank alive while eliminating opponents. The last surviving tank is declared the winner.

- **Optional Player Mode**: Players can opt to play against their own AI-driven tank to test and refine strategies.

This game engine serves as the foundation for participants to develop, test, and refine their AI models in a competitive yet accessible environment.

# Command-Line Arguments for `tank_game`

The `tank_game` engine allows customization of game settings through various command-line arguments. Below is a list of all available optional arguments, along with their default values if not specified.

## Optional Arguments

### `-r` or `-render`
- **Description**: Enables or disables rendering in the game engine.
- **Values**:
  - `t` or `true` – enables rendering.
  - `f` or `false` – disables rendering (headless mode).
- **Default**: `true` (Rendering is enabled by default).

---

### `-p1` or `-player_1`
- **Description**: Specifies the control scheme for Player 1.
- **Values**:
  - `wasd` – controls Player 1 using the `W`, `A`, `S`, `D` keys.
  - `arrow` – controls Player 1 using the arrow keys.
  - `<IP>:<Port>` – controls Player 1 using a REST API running on the specified IP address and port.
- **Default**: `wasd` (Player 1 uses `W`, `A`, `S`, `D` keys by default).

---

### `-p2` or `-player_2`
- **Description**: Specifies the control scheme for Player 2.
- **Values**:
  - `wasd` – controls Player 2 using the `W`, `A`, `S`, `D` keys.
  - `arrow` – controls Player 2 using the arrow keys.
  - `<IP>:<Port>` – controls Player 2 using a REST API running on the specified IP address and port.
- **Default**: `arrow` (Player 2 uses arrow keys by default).

---

## Usage Example

```bash
# Run the game with default settings (rendering enabled, Player 1 using WASD, Player 2 using arrow keys)
./tank_game

# Run the game in headless mode (no rendering)
./tank_game -r false

# Set Player 1 to use WASD keys and Player 2 to use a REST API at 127.0.0.1:8080
./tank_game -p1 wasd -p2 127.0.0.1:8080
```

# Todo

# Game Engine To-Do List

## Rendering
- [X] **Optional Rendering**  
  - Implement **headless mode** for the game engine, to run without rendering.
  - Set up **default rendering** mode with visual outputs.

## User Input Processing
- [X] **Movement**  
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
- [X] **Wall and Tank Collisions**  
  - Implement collision detection to prevent tanks from moving through walls and other tanks.

## Player Inputs
- [X] **Keyboard Inputs**  
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
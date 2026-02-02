# Wave Survival — Rust

A fast-paced wave survival game built entirely in Rust. Features real-time combat, wave-based progression, and retro pixel graphics.

![Game Preview](https://img.shields.io/badge/Game-Wave%20Survival-blue) ![Language](https://img.shields.io/badge/Language-Rust-orange) ![Engine](https://img.shields.io/badge/Engine-None-red)

## 🎮 Game Features

- **Wave-based Combat**: Survive increasingly difficult waves of enemies
- **Real-time Action**: Fast-paced gameplay with smooth 60 FPS rendering
- **Player Health System**: Start with 100 HP, avoid enemy collisions
- **Progressive Difficulty**: Each wave spawns more enemies (6 + 4 per wave)
- **Score System**: Track your survival progress
- **Retro Graphics**: Pixel-perfect rendering with custom graphics routines

## 🕹️ Controls

- **WASD** or **Arrow Keys**: Move player
- **Left Mouse Button**: Shoot bullets
- **Enter**: Reset game (when game over)

## 🚀 Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)
- macOS, Linux, or Windows

### Installation & Running

1. **Clone the repository**
   ```bash
   git clone https://github.com/Heddlok/RustWaveSurvival.git
   cd RustWaveSurvival
   ```

2. **Build and run the game**
   ```bash
   cargo run --release
   ```

3. **Build for release**
   ```bash
   cargo build --release
   ```

The release binary will be available at `target/release/wave_survival_rust`

## 🎯 How to Play

1. **Movement**: Use WASD or arrow keys to move your player (blue circle)
2. **Combat**: Click and hold the left mouse button to shoot bullets
3. **Survival**: Avoid touching enemies (red circles) to prevent damage
4. **Wave Progression**: Survive each wave to advance to the next level
5. **Scoring**: Your score increases as you survive longer and defeat enemies

## 🏗️ Technical Details

### Architecture

The game is built with a modular architecture:

- **`main.rs`**: Application entry point and main game loop
- **`game.rs`**: Core game logic and state management
- **`entities.rs`**: Player, Enemy, and Bullet definitions
- **`input.rs`**: Input handling system
- **`render.rs`**: Custom graphics rendering functions
- **`math.rs`**: Vector mathematics and utility functions
- **`config.rs`**: Game configuration constants

### Dependencies

- **`minifb`**: Cross-platform windowing and input handling
- **`rand`**: Random number generation for enemy spawning

### Performance

- **Target FPS**: 60 FPS (16.6ms frame time)
- **Resolution**: 900x720 pixels
- **Memory**: Efficient object pooling for bullets and enemies
- **Delta Time**: Frame-rate independent game logic

## 🛠️ Development

### Project Structure

```
src/
├── main.rs      # Application entry point
├── game.rs      # Core game logic
├── entities.rs  # Game entities (Player, Enemy, Bullet)
├── input.rs     # Input handling
├── render.rs    # Graphics rendering
├── math.rs      # Math utilities
└── config.rs    # Game configuration
```

### Building from Source

```bash
# Debug build
cargo build

# Release build (recommended for performance)
cargo build --release

# Run with optimizations
cargo run --release
```

## 🎨 Game Mechanics

### Player
- **Health**: 100 HP
- **Movement Speed**: 300 pixels/second
- **Shooting Cooldown**: 0.12 seconds
- **Collision Radius**: 12 pixels

### Enemies
- **Spawn Pattern**: 6 + (4 × wave number) enemies per wave
- **AI**: Simple movement toward player
- **Health**: 1 HP (destroyed by single bullet)
- **Collision**: Damages player on contact

### Bullets
- **Speed**: Fast projectiles with limited lifetime
- **Damage**: Instant kill on enemies
- **Pooling**: Efficient memory management with pre-allocated vectors

## 🤝 Contributing

Contributions are welcome! Feel free to:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 License

This project is open source and available under the [MIT License](LICENSE).

## 🎯 Future Enhancements

Potential features for future development:

- [ ] Power-ups and upgrades
- [ ] Different enemy types
- [ ] Sound effects and music
- [ ] High score persistence
- [ ] Multiple difficulty levels
- [ ] Particle effects
- [ ] Menu system

## 🐛 Known Issues

- Game window cannot be resized (by design for consistent gameplay)
- No sound effects currently implemented

---

**Enjoy surviving the waves!** 🎮

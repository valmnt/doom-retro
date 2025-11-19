# 🦝 Raycoon  
**A minimal raycasting engine written in Rust.**

Raycoon is a small, modern, and educational **2.5D raycasting engine** inspired by classic FPS techniques.
It focuses on clarity, simplicity, and clean architecture: the core provides pure logic (raycasting, collisions, tile mapping), while rendering and input are handled externally.

## ✨ Features
- Minimal DDA-based raycasting engine  
- Tile-based world with configurable blocking tiles  
- Per-axis collision handling  
- Strict separation between **engine** and **renderer**  
- Optional **Macroquad renderer** provided as an example  
- Lightweight, hackable, and easy to understand

## 🤝 Contributing
Contributions are welcome!  
Please keep the engine strictly backend-agnostic.  
Rendering, input handling, and tooling must remain in external modules.

## 📜 License
MIT License.
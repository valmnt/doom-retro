<pre>
888d888 8888b.  .d8888b .d88b.  .d88b. 88888b.  
888P"      "88bd88P"   d88""88bd88""88b888 "88b 
888    .d888888888     888  888888  888888  888 
888    888  888Y88b.   Y88..88PY88..88P888  888 
888    "Y888888 "Y8888P "Y88P"  "Y88P" 888  888 
</pre>

<br />

<div style="font-size: 25px;">
    A minimal <b>raycasting engine</b>  written in Rust.
</div>

<br />

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
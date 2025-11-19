<pre>
88""Yb    db    Yb  dP  dP""b8  dP"Yb   dP"Yb  88b 88 
88__dP   dPYb    YbdP  dP   `" dP   Yb dP   Yb 88Yb88 
88"Yb   dP__Yb    8P   Yb      Yb   dP Yb   dP 88 Y88 
88  Yb dP""""Yb  dP     YboodP  YbodP   YbodP  88  Y8
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

# Immediate Mode Rendering Practice

Experimenting with some ideas for 3d fantasy consoles and immediate mode APIs...

TODO:
- Add Lighting
- Clean up duplicated code, in creation of multiple render pipelines
    - Maybe move them to Pipeline.rs
- Consider "buffer" management or "draw preloaded asset" via buffers
- Consider how to handle dynamic or procedural textures
- Add 2d quad rendering


Ideas:
- Single "immediate mode" geometry and texture buffer always mapped to specific addresses
- Dynamic textures can store a pointer & length to texture data on CPU side
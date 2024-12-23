# Immediate Mode Rendering Practice

Experimenting with some ideas for 3d fantasy consoles and immediate mode APIs...

DOING:

TODO:
- Clean up App/State/VGpu More...
  - Move Surface out of App into VGPU
- Improve Lighting
  - Add Attenuation/Range Falloff
  - Add spotlight
  - Consider a special slot for ambient and directional light, to support 8 + 2 total lights
    - Maybe not needed with improved ambient and lighting models
  - Adjust lighting to include a light mask
    - Could be tied to instance data
    - Prevents weird async issue of setting lights and drawing meshes out of order
- Add size/memory limits for VirtualGPU
- Make Camera / View Matrix setup stuff available from Game
- Consider how to handle dynamic or procedural textures
- Support multiple viewports
  - figure out API for this
  - could include Scissor rect if necessary


Longer Term Ideas:
- Single "immediate mode" geometry and texture buffer always mapped to specific addresses
- Dynamic textures can store a pointer & length to texture data on CPU side
- Procedural Environment Maps
  - Reflections for Metallic Surfaces
  - IBR image based lighting


Implementation Notes:
For 2d Quad Rendering...
- Z -1.0 is "Closest to the screen"
- Z 1.0 is "Farthest from the screen"
- Any values less than -1, or greater than 1.0, wont be drawn as they fall outside of NDC coordinates
- Prefer lower values instead of 0.0 as this can intersect with 3d geometry
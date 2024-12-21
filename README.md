# Immediate Mode Rendering Practice

Experimenting with some ideas for 3d fantasy consoles and immediate mode APIs...

DOING:
- Add 2d quad rendering

TODO:
- Improve Lighting
  - Add Attenuation/Range Falloff
  - Add spotlight
  - Consider a special slot for ambient and directional light, to support 8 + 2 total lights
    - Maybe not needed with improved ambient and lighting models
  - Adjust lighting to include a light mask
    - Could be tied to instance data
    - Prevents weird async issue of setting lights and drawing meshes out of order
- Move stuff from app State over to VirtualGpu
  - Add size/memory limits
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
# Immediate Mode Rendering Practice

Experimenting with some ideas for 3d fantasy consoles and immediate mode APIs...

TODO:
- Improve Lighting
  - Add Attenuation/Range Falloff
  - Consider a special slot for ambient and directional light, to support 8 + 2 total lights
- Move stuff from app State over to VirtualGpu
  - Add size/memory limits
- Make Camera / View Matrix setup stuff available from Game
- Consider how to handle dynamic or procedural textures
- Add 2d quad rendering
- Support multiple viewports
  - figure out API for this
  - could include Scissor rect if necessary

- Consider Shader, simplified specular rewrite
  - Ward BRDF over current one
  - Schlick Phong
  - NdotL "Simplified Specular" (Custom Retro Look)

Ideas:
- Single "immediate mode" geometry and texture buffer always mapped to specific addresses
- Dynamic textures can store a pointer & length to texture data on CPU side
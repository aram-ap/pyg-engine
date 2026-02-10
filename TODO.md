# TODO
Items in order from Highest->Lowest priority.

- Draw Circles, Rectangles, Polygons, Text, Images
    - Can be attached to GameObject items
      - Direct mode allows for drawing based on the pixel value
    - Transforms (Move them, resize them, rotate them)
    - Layers ( Background (0), 1, 2, ..., 254, Foreground(255) );
        - Higher value = Higher precedence
        - Subject to change?
    - Z axis
    - Color and opacity
    - Enable/disable
    - Text
        - Glyphon or wgpu_text
    - Decorations
        - Line color, fill color
        - Line width

- GameObject Components
    - Transforms (DEFAULT)
        - (moves the drawn item too)
    - Drawable (Shapes/Images)
    - Rigidbody system
        - Physics materials
    - Audio Listener/Source
    - Script attachments

- Groupings (Name Ideas: Bunch, Box, or Frame)
    - One singular parent object handles the primary world+GameObject framing.
        - Means that groupings can have nested groupings.
    - Holds a bunch of GameObjects
    - Group enable/disable/delete
    - Positions
    - Group transforms
    - Pivot alignment

- Physics system
  - Rigid body collisions
  - World gravity
  - Collision triggering
  - Mass/Momentum/Velocity/Inertia

- Input system
  - Single easy API for getting button state, mouse locations
  - Axes system 
    - Eventually make it readdressable
  - Joysticks (eventually)
    - Deadzone areas
    - Sensitivity

- Camera (GameObject)
    - Added by default
    - Control world size
    - Moving the camera does world transform
        - Rotating would require a matrix transformation
    - Clipping layers
    - Background color
    - Color mask (Fills the window with the color)

- Scene
    - Loadable/saveable
        - Serialized in YAML
    - All game objects, components, groupings, values, etc
    - Assets
        - Links to all images, audio, scripts
    - Hash checks (SHA-256 Checksum)
     
- Python Bindings:
    - The primary use-case
    - Same gameobject/drawing ability

- Global Event system 
  - Event source/listener system
  - Priority queue

- UI Canvas
  - Buttons, Panels, Text Blocks, Text Boxes
  - Buttons:
    - Momentary button
    - Toggle button
    - Radial buttons
    - Checkbox
  - Panels:
    - Filled/unfilled, borders, colors
    - Rounded corners
    - Images
  - Text Blocks:
  - Text Boxes:
    - Editable by clicking on it.

- Coroutines
    - Async events

- Pyg-Editor
    - Game engine with a GUI
      - using ICED GUI library
    - Project creation
    - Live scene view
    - GameObject Heirarchy
        - Serialized values
    - Play/Pause/Stop
    - Build system (PyPy compilation)

Future:
    - In-Built Networking
    - Particle system

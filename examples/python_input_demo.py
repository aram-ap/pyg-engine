import random

from pyg_engine import Color, Engine, MouseButton

def main():
    engine = Engine()
    engine.start_manual(title="Input Demo", width=800, height=600)
    
    # Simple state for demonstration
    circle_x = 400.0
    circle_y = 300.0
    circle_radius = 20.0
    circle_color = Color(1.0, 0.0, 0.0, 1.0)
    
    speed = 200.0  # pixels per second

    print("Controls:")
    print("  WASD / Arrow Keys: Move circle")
    print("  Space: Change color")
    print("  Click: Teleport circle")
    print("  Escape: Quit")

    while engine.poll_events():
        # Get delta time for smooth movement
        dt = engine.delta_time
        if dt > 0.1:
            dt = 0.1  # Cap delta time
        
        # Axis Input (default Unity-style mappings are configured in the engine)
        horizontal = engine.input.axis("Horizontal")
        vertical = engine.input.axis("Vertical")
        circle_x += horizontal * speed * dt
        circle_y -= vertical * speed * dt
            
        if engine.input.action_pressed("jump"):
            # Toggle color
            if circle_color.r > 0.5:
                circle_color = Color(0.0, 0.0, 1.0, 1.0)
            else:
                circle_color = Color(1.0, 0.0, 0.0, 1.0)
                
        if engine.input.action_pressed("escape"):
            break
            
        # Mouse Input
        if engine.input.mouse_button_pressed(MouseButton.LEFT):
            mx, my = engine.input.mouse_position
            circle_x = mx
            circle_y = my
            print(f"Teleported to mouse: {mx}, {my}")

        if engine.input.mouse_button_pressed(MouseButton.RIGHT):
            # get display size
            display_width, display_height = engine.get_display_size()

            # get random position within display size
            rx = random.uniform(0, display_width)
            ry = random.uniform(0, display_height)
            circle_x = rx
            circle_y = ry
            print(f"Teleported to random position: {rx}, {ry}")
            
            
        # Axis Input
        # horizontal = engine.input.axis("Horizontal")
        # vertical = engine.input.axis("Vertical")
        # circle_x += horizontal * speed * dt
        # circle_y += vertical * speed * dt

        # Rendering
        engine.clear_draw_commands()
        engine.draw_circle(circle_x, circle_y, circle_radius, circle_color)
        engine.update()
        engine.render()

if __name__ == "__main__":
    main()

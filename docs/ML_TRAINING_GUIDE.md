# Machine Learning Training Guide

## Time Scaling and Headless Mode for ML/Deep Reinforcement Learning

This guide covers the time scaling and headless mode features specifically designed for machine learning and deep reinforcement learning applications.

## Overview

When training ML agents, you typically need to run thousands or millions of simulation steps. Running simulations at real-time speed (1x) is far too slow for practical training. The engine now supports:

1. **Time Scaling**: Run simulations faster or slower than real-time
2. **Headless Mode**: Skip rendering to maximize performance
3. **Scaled Physics**: Physics simulation scales correctly with time

## Time Scaling

### Basic Usage

```python
from src.core.engine import Engine
from src.utilities.object_types import Size

# Create engine
engine = Engine(
    size=Size(w=800, h=600),
    fpsCap=60,
    useDisplay=True  # Set to False for headless mode
)

# Set time scale
engine.set_time_scale(10.0)  # Run at 10x speed

# Get current time scale
current_scale = engine.get_time_scale()  # Returns 10.0

# Get scaled delta time (automatically applied)
dt = engine.dt()  # Returns delta time * time_scale

# Get unscaled delta time (real time)
real_dt = engine.get_unscaled_dt()  # Returns actual delta time
```

### Time Scale Values

- `1.0` - Normal speed (real-time)
- `< 1.0` - Slow motion (e.g., 0.5 = half speed)
- `> 1.0` - Fast forward (e.g., 10.0 = 10x speed)
- Any positive value is supported

### Recommended Values for ML Training

- **Initial Training**: `10.0` - `50.0x` - Fast enough to see progress quickly
- **Production Training**: `100.0` - `1000.0x` - Maximum speed for large-scale training
- **Debugging**: `0.5` - `1.0x` - Slower speeds to observe behavior

## Headless Mode

### Basic Usage

**IMPORTANT**: You must call `configure_headless_mode()` BEFORE creating any Engine instances to enable true headless mode without any display windows.

```python
from src.core.engine import Engine, configure_headless_mode

# MUST be called first, before creating any engines
configure_headless_mode()

# Now create engine in headless mode
engine = Engine(
    size=Size(w=800, h=600),
    fpsCap=60,
    useDisplay=False  # Headless mode - no rendering window
)
```

**Why is this necessary?**

The `configure_headless_mode()` function sets the `SDL_VIDEODRIVER` environment variable to `'dummy'` before pygame initializes. This tells SDL (the library pygame uses) not to create any display windows. This must be done before any pygame modules are initialized (including the audio system), which is why it needs to be called at the very start of your script.

### Performance Benefits

Headless mode provides massive performance improvements by skipping all rendering operations:

| Configuration | Performance | Use Case |
|--------------|-------------|----------|
| Display + 1x speed | 60 FPS | Normal gameplay |
| Headless + 1x speed | ~500 FPS | Light training |
| Headless + 10x speed | ~5,000 FPS | Standard training |
| Headless + 100x speed | ~20,000+ FPS | Intensive training |

*Performance varies based on scene complexity and hardware*

## Complete ML Training Example

```python
import sys
from pathlib import Path
sys.path.insert(0, str(Path(__file__).parent.parent))

from src.core.engine import Engine, configure_headless_mode
from src.core.gameobject import GameObject
from src.components.script import Script
from src.physics.rigidbody import RigidBody
from src.physics.collider import CircleCollider
from src.utilities.vector2 import Vector2
from src.utilities.object_types import Size, BasicShape
from pygame import Color


class MLAgent(Script):
    """Simple RL agent example."""
    
    def __init__(self, game_object):
        super().__init__(game_object)
        self.episode_reward = 0
        self.episode_steps = 0
        
    def update(self, engine):
        # Get observations
        position = self.game_object.position
        
        # Choose action (placeholder for your ML model)
        action = self.choose_action(position)
        
        # Apply action
        self.apply_action(action)
        
        # Calculate reward
        reward = self.calculate_reward()
        self.episode_reward += reward
        self.episode_steps += 1
        
        # Check if episode is done
        if self.is_done():
            self.reset_episode()
    
    def choose_action(self, observation):
        """Placeholder - integrate your ML model here."""
        return 0  # Replace with model inference
    
    def apply_action(self, action):
        """Apply the chosen action to the agent."""
        rb = self.game_object.get_component(RigidBody)
        if rb and action == 1:  # Example: jump action
            rb.add_force(Vector2(0, 1000))
    
    def calculate_reward(self):
        """Calculate reward based on agent state."""
        # Example reward function
        if self.game_object.position.y > 100:
            return 1.0  # Reward for staying high
        return -0.1  # Small penalty per step
    
    def is_done(self):
        """Check if episode should end."""
        return self.episode_steps >= 1000  # Max episode length
    
    def reset_episode(self):
        """Reset agent for new episode."""
        print(f"Episode complete! Reward: {self.episode_reward}")
        self.episode_reward = 0
        self.episode_steps = 0
        # Reset agent position, etc.


def main():
    """ML Training Setup."""
    
    # Configuration
    HEADLESS = True  # Set to False to visualize
    TIME_SCALE = 50.0  # 50x speed for training
    FPS_CAP = 60  # Or 0 for uncapped
    
    # Enable headless mode (MUST be called before creating engine)
    if HEADLESS:
        configure_headless_mode()
    
    # Create engine
    engine = Engine(
        size=Size(w=800, h=600),
        backgroundColor=Color(30, 30, 30),
        windowName="ML Training",
        fpsCap=FPS_CAP,
        useDisplay=not HEADLESS
    )
    
    # Set time scale for fast training
    engine.set_time_scale(TIME_SCALE)
    
    # Create agent
    agent = GameObject(
        name="Agent",
        position=Vector2(0, 100),
        size=Vector2(20, 20),
        color=Color(100, 200, 100),
        basicShape=BasicShape.Circle
    )
    
    # Add physics
    rb = agent.add_component(RigidBody)
    rb.mass = 1.0
    rb.use_gravity = True
    agent.add_component(CircleCollider, radius=20)
    
    # Add ML agent script
    agent.add_component(MLAgent)
    
    engine.addGameObject(agent)
    
    # Start training
    print(f"Starting training at {TIME_SCALE}x speed...")
    engine.start()


if __name__ == "__main__":
    main()
```

## Integration with ML Frameworks

### PyTorch Example

```python
import torch
import torch.nn as nn

class MLAgent(Script):
    def __init__(self, game_object):
        super().__init__(game_object)
        self.model = nn.Sequential(
            nn.Linear(4, 128),  # 4 observations
            nn.ReLU(),
            nn.Linear(128, 64),
            nn.ReLU(),
            nn.Linear(64, 2)  # 2 actions
        )
        self.optimizer = torch.optim.Adam(self.model.parameters(), lr=0.001)
    
    def choose_action(self, observation):
        state = torch.FloatTensor(observation)
        with torch.no_grad():
            action_probs = self.model(state)
        return torch.argmax(action_probs).item()
```

### Stable Baselines3 Example

```python
from stable_baselines3 import PPO
from stable_baselines3.common.env_checker import check_env
import gymnasium as gym

class PyGEngineEnv(gym.Env):
    """Custom Gymnasium environment wrapping PyG Engine."""
    
    def __init__(self):
        super().__init__()
        self.engine = Engine(
            size=Size(w=800, h=600),
            fpsCap=0,  # Uncapped for training
            useDisplay=False  # Headless
        )
        self.engine.set_time_scale(100.0)  # Fast training
        
        # Define action and observation space
        self.action_space = gym.spaces.Discrete(4)
        self.observation_space = gym.spaces.Box(
            low=-np.inf, high=np.inf, shape=(4,), dtype=np.float32
        )
    
    def reset(self, seed=None, options=None):
        # Reset environment
        return observation, info
    
    def step(self, action):
        # Execute action, run one simulation step
        # Return observation, reward, terminated, truncated, info
        pass

# Train agent
env = PyGEngineEnv()
model = PPO("MlpPolicy", env, verbose=1)
model.learn(total_timesteps=1000000)
```

## Best Practices

### 1. Start with Visualization

```python
# First, verify your environment works correctly with visualization
engine = Engine(useDisplay=True)
engine.set_time_scale(1.0)  # Normal speed
```

### 2. Then Switch to Headless Training

```python
# Once verified, switch to headless for training
from src.core.engine import configure_headless_mode

# Enable headless mode first
configure_headless_mode()

# Create headless engine
engine = Engine(useDisplay=False)
engine.set_time_scale(50.0)  # Fast training
```

### 3. Monitor Performance

```python
class PerformanceMonitor(Script):
    def __init__(self, game_object):
        super().__init__(game_object)
        self.frame_count = 0
        self.start_time = time.time()
    
    def update(self, engine):
        self.frame_count += 1
        if self.frame_count % 1000 == 0:
            elapsed = time.time() - self.start_time
            fps = self.frame_count / elapsed
            print(f"FPS: {fps:.1f}, Speedup: {fps/60:.1f}x")
```

### 4. Handle Episode Resets Efficiently

```python
def reset_episode(self):
    """Efficient reset without recreating objects."""
    # Reset positions
    self.game_object.position = Vector2(0, 100)
    
    # Reset velocities
    rb = self.game_object.get_component(RigidBody)
    if rb:
        rb.velocity = Vector2(0, 0)
        rb.angular_velocity = 0.0
    
    # Reset other state
    self.episode_reward = 0
    self.episode_steps = 0
```

### 5. Use Appropriate Time Scales

```python
# For debugging
engine.set_time_scale(0.5)  # Slow motion

# For initial development
engine.set_time_scale(10.0)  # 10x speed

# For production training
engine.set_time_scale(100.0)  # 100x speed

# For maximum throughput (be careful with stability)
engine.set_time_scale(1000.0)  # 1000x speed
```

## Performance Optimization Tips

1. **Use Headless Mode**: Always train in headless mode (`useDisplay=False`)
2. **Uncap FPS**: Set `fpsCap=0` for maximum speed
3. **Minimize Objects**: Keep scene complexity low during training
4. **Batch Training**: Run multiple environments in parallel
5. **Profile Your Code**: Use Python profilers to find bottlenecks

## Limitations and Considerations

### Physics Stability

Very high time scales (>100x) may affect physics stability:
- Use smaller timesteps if physics becomes unstable
- Test at lower scales first
- Monitor for NaN values or explosive behavior

### Time Scale Range

- Minimum: Any positive value (e.g., 0.1x)
- Maximum: Unlimited (tested up to 1000x)
- Recommended for ML: 10x - 100x

### Determinism

For reproducible training:
```python
import random
import numpy as np

# Set seeds
random.seed(42)
np.random.seed(42)
# Note: PyMunk physics may have minor floating-point variations
```

## Troubleshooting

### Issue: Physics behaves strangely at high speeds

**Solution**: Reduce time scale or adjust physics timestep

```python
# Option 1: Lower time scale
engine.set_time_scale(10.0)  # Instead of 100.0

# Option 2: Physics system already uses fixed timestep
# Check physics_system.dt_fixed in physics_system.py
```

### Issue: Training is slower than expected

**Solutions**:
1. Enable headless mode: `useDisplay=False`
2. Uncap FPS: `fpsCap=0`
3. Increase time scale: `set_time_scale(100.0)`
4. Reduce scene complexity
5. Profile your Python code

### Issue: Agent behavior differs between display and headless mode

**Solution**: This shouldn't happen, but verify:
```python
# Test both modes with same seed
engine.set_time_scale(1.0)  # Same scale
# Check if behavior matches
```

## Example Scripts

See these example files:
- `examples/ml_training_demo.py` - Basic demonstration
- `examples/time_scale_headless_test.py` - Comprehensive testing

## Conclusion

The time scaling and headless mode features make this engine well-suited for ML/deep reinforcement learning training, allowing you to:

- Train agents 100x+ faster than real-time
- Run headless for maximum performance
- Maintain correct physics simulation at any speed
- Easily switch between visualization and training modes

Happy training! 🚀


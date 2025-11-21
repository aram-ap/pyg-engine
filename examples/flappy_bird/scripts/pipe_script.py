"""
Pipe Script for Flappy Bird
Controls pipe movement and scoring
"""

from pyg_engine import Script, Vector2

class PipeScript(Script):
    """Script for controlling pipes in Flappy Bird."""

    def __init__(self, game_object, controller=None, speed=200, is_score_pipe=False):
        super().__init__(game_object)
        
        # Reference to game controller
        self.controller = controller
        
        # Movement properties
        self.speed = speed  # Pixels per second
        
        # Scoring
        self.is_score_pipe = is_score_pipe  # Only upper pipe scores
        self.has_scored = False

    def start(self, engine):
        """Called when the script starts."""
        super().start(engine)

    def update(self, engine):
        """Update pipe movement."""
        if not self.controller:
            return
        
        dt = engine.dt()
        
        # Only move when playing
        if self.controller.game_state == "playing":
            # Move pipe left
            self.game_object.position.x -= self.speed * dt
            
            # Check if bird passed this pipe for scoring
            if self.is_score_pipe and not self.has_scored:
                if self.controller.bird and \
                   self.game_object.position.x < self.controller.bird.position.x:
                    self.controller.add_score()
                    self.has_scored = True

"""
Example demonstrating the Logger functionality in pyg_engine.
The Logger is automatically initialized when creating a Core instance.
"""

import pyg

def main():
    print("Testing pyg_engine Logger...")

    # Create Engine instance - this will initialize the logger and log messages
    engine = pyg.Engine()

    # Get version (logger will show initialization messages in console)
    version = engine.version
    print(f"Engine Version: {version}")

    # Simulate some updates
    # for i in range(3):
    #     engine.update(0.016)  # ~60 FPS

    # Cleanup - this will log shutdown messages
    # engine.on_destroy()

    print("Logger test complete! Check the console output above for log messages.")

if __name__ == "__main__":
    main()


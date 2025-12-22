#!/usr/bin/env python3
"""
Demonstration of the pyg_engine logging system.

This script showcases all logging features including:
- Different log levels
- Console and file logging
- Timestamps
- Custom configurations
"""

import pyg_engine as pyg
import tempfile
from pathlib import Path


def demo_basic_logging():
    """Demonstrate basic console logging."""
    print("=" * 60)
    print("DEMO 1: Basic Console Logging")
    print("=" * 60)
    
    engine = pyg.Engine()
    
    print("\nLogging at different levels:\n")
    engine.log_info("This is an INFO message")
    engine.log_warn("This is a WARNING message")
    engine.log_error("This is an ERROR message")
    engine.log("Using default log() method (INFO level)")
    
    print("\n" + "=" * 60 + "\n")


def demo_log_levels():
    """Demonstrate different log level configurations."""
    print("=" * 60)
    print("DEMO 2: Log Level Filtering")
    print("=" * 60)
    
    print("\nWith DEBUG level (shows debug and above):\n")
    engine_debug = pyg.Engine(log_level="DEBUG")
    engine_debug.log_debug("DEBUG: This will appear")
    engine_debug.log_info("INFO: This will appear")
    engine_debug.log_warn("WARN: This will appear")
    
    print("\nWith WARN level (shows warnings and errors only):\n")
    engine_warn = pyg.Engine(log_level="WARN")
    engine_warn.log_info("INFO: This will NOT appear")
    engine_warn.log_warn("WARN: This WILL appear")
    engine_warn.log_error("ERROR: This WILL appear")
    
    print("\n" + "=" * 60 + "\n")


def demo_file_logging():
    """Demonstrate file logging with rotation."""
    print("=" * 60)
    print("DEMO 3: File Logging")
    print("=" * 60)
    
    # Create a temporary directory for logs
    with tempfile.TemporaryDirectory() as tmpdir:
        log_dir = Path(tmpdir) / "demo_logs"
        
        print(f"\nLogging to directory: {log_dir}\n")
        
        engine = pyg.Engine(
            enable_file_logging=True,
            log_directory=str(log_dir),
            log_level="INFO"
        )
        
        # Log some messages
        engine.log_info("This message goes to BOTH console and file")
        engine.log_warn("Warning: Also in both locations")
        engine.log_error("Error: Saved for later analysis")
        
        # Give it a moment to flush
        import time
        time.sleep(0.1)
        
        # Check if log files were created
        if log_dir.exists():
            log_files = list(log_dir.glob("*.log*"))
            print(f"\n✓ Created {len(log_files)} log file(s):")
            for log_file in log_files:
                print(f"  - {log_file.name} ({log_file.stat().st_size} bytes)")
        else:
            print("\nNote: Log directory creation is handled asynchronously")
    
    print("\n" + "=" * 60 + "\n")


def demo_trace_level():
    """Demonstrate TRACE level logging (very verbose)."""
    print("=" * 60)
    print("DEMO 4: TRACE Level (Most Verbose)")
    print("=" * 60)
    
    print("\nTRACE level shows everything:\n")
    engine = pyg.Engine(log_level="TRACE")
    
    engine.log_trace("TRACE: Entering function")
    engine.log_debug("DEBUG: Variable x = 42")
    engine.log_info("INFO: Operation completed")
    engine.log_trace("TRACE: Exiting function")
    
    print("\n" + "=" * 60 + "\n")


def demo_real_world_usage():
    """Demonstrate realistic game engine usage."""
    print("=" * 60)
    print("DEMO 5: Real-World Game Engine Usage")
    print("=" * 60)
    
    print("\nSimulating a game initialization:\n")
    
    engine = pyg.Engine(log_level="INFO")
    
    # Startup sequence
    engine.log_info("pyg_engine starting...")
    engine.log_info(f"Version: {engine.version}")
    
    # Simulated game loading
    assets = ["player.png", "enemy.png", "background.png"]
    engine.log_info(f"Loading {len(assets)} assets...")
    
    for asset in assets:
        engine.log_debug(f"Loading asset: {asset}")  # Won't show (INFO level)
    
    engine.log_info("Assets loaded successfully")
    
    # Simulated warning
    engine.log_warn("Texture quality reduced to fit in memory")
    
    # Simulated error
    try:
        # Simulate an error
        raise FileNotFoundError("level_2.dat")
    except Exception as e:
        engine.log_error(f"Failed to load level: {e}")
    
    engine.log_info("Engine initialized and ready")
    
    print("\n" + "=" * 60 + "\n")


def main():
    """Run all demonstrations."""
    print("\n")
    print("╔" + "=" * 58 + "╗")
    print("║" + " " * 58 + "║")
    print("║" + "  pyg_engine Logging System Demonstration".center(58) + "║")
    print("║" + f"  Version {pyg.Engine().version}".center(58) + "║")
    print("║" + " " * 58 + "║")
    print("╚" + "=" * 58 + "╝")
    print("\n")
    
    try:
        demo_basic_logging()
        demo_log_levels()
        demo_file_logging()
        demo_trace_level()
        demo_real_world_usage()
        
        print("╔" + "=" * 58 + "╗")
        print("║" + " " * 58 + "║")
        print("║" + "  All demonstrations completed successfully!".center(58) + "║")
        print("║" + " " * 58 + "║")
        print("║" + "  For more information, see LOGGING_GUIDE.md".center(58) + "║")
        print("║" + " " * 58 + "║")
        print("╚" + "=" * 58 + "╝")
        print("\n")
        
    except Exception as e:
        print(f"\n❌ Error during demonstration: {e}\n")
        raise


if __name__ == "__main__":
    main()

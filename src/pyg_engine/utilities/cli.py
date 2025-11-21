#!/usr/bin/env python3
"""
Command-line interface for Pyg Engine
"""

import argparse
import sys
from pathlib import Path

def main():
    """Main CLI entry point"""
    parser = argparse.ArgumentParser(
        description="Pyg Engine - A Python Game Engine",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  pyg-engine --version
  pyg-engine run examples/main.py
  pyg-engine test
        """
    )
    
    parser.add_argument(
        '--version',
        action='version',
        version='%(prog)s 1.0.0'
    )
    
    subparsers = parser.add_subparsers(dest='command', help='Available commands')
    
    # Run command
    run_parser = subparsers.add_parser('run', help='Run a game script')
    run_parser.add_argument('script', help='Path to the game script to run')
    
    # Test command
    test_parser = subparsers.add_parser('test', help='Run the test suite')
    
    # Examples command
    examples_parser = subparsers.add_parser('examples', help='List available examples')
    
    args = parser.parse_args()
    
    if args.command == 'run':
        run_script(args.script)
    elif args.command == 'test':
        run_tests()
    elif args.command == 'examples':
        list_examples()
    else:
        parser.print_help()

def run_script(script_path):
    """Execute a game script file."""
    try:
        script_file = Path(script_path)
        if not script_file.exists():
            print(f"Error: Script file '{script_path}' not found")
            sys.exit(1)
        
        # Execute the script
        exec(script_file.read_text())
        
    except Exception as e:
        print(f"Error running script: {e}")
        sys.exit(1)

def run_tests():
    """Execute the test suite using pytest."""
    try:
        import pytest
        import sys
        from pathlib import Path
        
        # Get the tests directory
        tests_dir = Path(__file__).parent.parent / 'tests'
        
        if not tests_dir.exists():
            print("Error: Tests directory not found")
            sys.exit(1)
        
        # Run pytest
        sys.argv = ['pytest', str(tests_dir)]
        pytest.main()
        
    except ImportError:
        print("Error: pytest not installed. Install with: pip install pytest")
        sys.exit(1)
    except Exception as e:
        print(f"Error running tests: {e}")
        sys.exit(1)

def list_examples():
    """Display available example scripts."""
    try:
        examples_dir = Path(__file__).parent.parent / 'examples'
        
        if not examples_dir.exists():
            print("Error: Examples directory not found")
            return
        
        print("Available examples:")
        for example_file in examples_dir.glob('*.py'):
            print(f"  - {example_file.name}")
            
    except Exception as e:
        print(f"Error listing examples: {e}")

if __name__ == '__main__':
    main() 
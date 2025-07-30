import subprocess
import signal
import sys

class ScriptRunner:
    """Manages execution of external Python scripts."""
    
    def __init__(self, scripts=None):
        self.scripts = scripts or []
        self.processes = []

    def add_script(self, script_path):
        """Add a script to the execution queue."""
        self.scripts.append(script_path)

    def run_scripts(self):
        """Execute all queued scripts concurrently."""
        self.processes = []
        for script in self.scripts:
            try:
                print(f"Running {script}...")
                proc = subprocess.Popen([sys.executable, script], stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
                self.processes.append(proc)
            except Exception as e:
                print(f"Failed to start {script}: {e}")

        # Collect output after all processes started
        for proc in self.processes:
            try:
                stdout, stderr = proc.communicate()
                print(f"Output of {proc.args[1]}:\n{stdout}")
                if stderr:
                    print(f"Errors from {proc.args[1]}:\n{stderr}")
            except Exception as e:
                print(f"Error communicating with process: {e}")

    def kill_all(self):
        """Terminate all running script processes."""
        print("Killing all running script processes...")
        for proc in self.processes:
            if proc.poll() is None:  # Process is still running
                try:
                    proc.terminate()  # Graceful termination
                    proc.wait(timeout=5)
                except subprocess.TimeoutExpired:
                    print(f"Process {proc.args[1]} did not terminate, killing now.")
                    proc.kill()  # Force kill
                except Exception as e:
                    print(f"Error killing process {proc.args[1]}: {e}")
        self.processes = []

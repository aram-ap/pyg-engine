import pyg_engine
from pyg_engine import Engine, Priority, GlobalDictionary

# To run the engine without a display, you may use the useDisplay modifier
engine = Engine(useDisplay = False)

# Globals or objects can be used to do things with it
engine.globals.set('num', 0)
engine.globals.set('max_num', 20)

# Make sure to add some kind of stop system, unless you're ok with it running indefinitely
# This program is set to stop the program when the max num is reached
def stop_after_max(engine):
    num = engine.globals.get('num')
    max = engine.globals.get('max_num')
    if(num > max):
        engine.stop()

# This is a separate runnable that manages iterating the global
def sayhi(engine):
    num = engine.globals.get('num')
    print("{}: hi!".format(num))
    num += 1
    engine.globals.set('num', num)

# Add your runnables or GameObjects and you're set to go.
engine.add_runnable(func=sayhi, max_runs = 100)
engine.add_runnable(func=stop_after_max)

engine.start()


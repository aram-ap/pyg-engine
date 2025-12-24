use std::time::SystemTime;
use super::logging;
/**
    The time class.
*/
pub struct Time {
    /**
        The system time.
    */    
    system_time: SystemTime,
    /**
        The time since the last tick.
    */
    delta_time: f32,
    /**
        The total time since the start of the application.
    */
    elapsed_time: f32,
    /**
        The fixed timestep.
    */
    fixed_timestep: f32,

    last_fixed_time: f32,
}

impl Time {
    /**
        Creates a new time instance.
        @return: The new time instance.
    */
    pub fn new() -> Self {
        Self {
            system_time: SystemTime::now(),
            delta_time: 0.0,
            elapsed_time: 0.0,
            fixed_timestep: 1.0, 
            last_fixed_time: 0.0,
        }
    }

    pub fn tick(&mut self) -> f32 {
        self.delta_time = self.system_time.elapsed().unwrap().as_secs_f32() - self.elapsed_time;
        self.elapsed_time = SystemTime::now().duration_since(self.system_time).unwrap_or_default().as_secs_f32();
        self.delta_time
    }

    pub fn tick_fixed(&mut self) -> (bool, f32) {
        if self.elapsed_time - self.last_fixed_time >= self.fixed_timestep {
            self.last_fixed_time = self.elapsed_time;
            (true, self.fixed_timestep)
        } else {
            (false, 0.0)
        }
    }

    pub fn last_fixed_time(&self) -> f32 {
        self.last_fixed_time
    }

    pub fn set_fixed_timestep(&mut self, fixed_timestep: f32) {
        self.fixed_timestep = fixed_timestep;
    }

    pub fn delta_time(&self) -> f32 {
        self.delta_time
    }

    pub fn fixed_timestep(&self) -> f32 {
        self.fixed_timestep
    }

    pub fn elapsed_time(&self) -> f32 {
        self.elapsed_time
    }

    pub fn log_info(&self) {
        logging::log_info("--------------------------------");
        logging::log_info("Engine update loop");
        logging::log_info(&format!("Delta time: {}", self.delta_time()));
        logging::log_info(&format!("Elapsed time: {}", self.elapsed_time()));
        logging::log_info(&format!("Fixed timestep: {}", self.fixed_timestep()));
        logging::log_info(&format!("Last fixed time: {}", self.last_fixed_time()));
        logging::log_info("--------------------------------");
    }
}
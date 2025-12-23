use std::time::SystemTime;

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
        }
    }

    pub fn tick(&mut self) {
        self.delta_time = self.system_time.elapsed().unwrap().as_secs_f32();
        self.elapsed_time = SystemTime::now().duration_since(self.system_time).unwrap_or_default().as_secs_f32();
    }

    pub fn delta_time(&self) -> f32 {
        self.delta_time
    }

    pub fn elapsed_time(&self) -> f32 {
        self.elapsed_time
    }
}
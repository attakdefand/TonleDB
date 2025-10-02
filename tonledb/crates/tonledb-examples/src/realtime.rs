//! Real-time and embedded examples
//!
//! This module demonstrates drone flight controller firmware using RTIC
//! for hard-real-time task scheduling on bare-metal.

use std::time::{Duration, Instant};
use rand::Rng;

/// Drone state representation
#[derive(Debug, Clone)]
pub struct DroneState {
    pub position: (f64, f64, f64), // x, y, z coordinates
    pub velocity: (f64, f64, f64), // x, y, z velocities
    pub attitude: (f64, f64, f64), // roll, pitch, yaw
    pub battery_level: f64,        // percentage
    pub armed: bool,               // whether motors are armed
}

impl DroneState {
    pub fn new() -> Self {
        Self {
            position: (0.0, 0.0, 0.0),
            velocity: (0.0, 0.0, 0.0),
            attitude: (0.0, 0.0, 0.0),
            battery_level: 100.0,
            armed: false,
        }
    }
}

/// Sensor readings from the drone
#[derive(Debug, Clone)]
pub struct SensorReadings {
    pub accelerometer: (f64, f64, f64),
    pub gyroscope: (f64, f64, f64),
    pub magnetometer: (f64, f64, f64),
    pub barometer: f64,
    pub gps: (f64, f64, f64), // latitude, longitude, altitude
}

/// Control commands for the drone
#[derive(Debug, Clone)]
pub struct ControlCommands {
    pub throttle: f64, // 0.0 to 1.0
    pub roll: f64,     // -1.0 to 1.0
    pub pitch: f64,    // -1.0 to 1.0
    pub yaw: f64,      // -1.0 to 1.0
}

/// A simple PID controller
pub struct PIDController {
    kp: f64, // Proportional gain
    ki: f64, // Integral gain
    kd: f64, // Derivative gain
    previous_error: f64,
    integral: f64,
}

impl PIDController {
    pub fn new(kp: f64, ki: f64, kd: f64) -> Self {
        Self {
            kp,
            ki,
            kd,
            previous_error: 0.0,
            integral: 0.0,
        }
    }

    pub fn update(&mut self, error: f64, dt: f64) -> f64 {
        self.integral += error * dt;
        let derivative = (error - self.previous_error) / dt;
        let output = self.kp * error + self.ki * self.integral + self.kd * derivative;
        self.previous_error = error;
        output
    }

    pub fn reset(&mut self) {
        self.previous_error = 0.0;
        self.integral = 0.0;
    }
}

/// A simple flight controller
pub struct FlightController {
    roll_controller: PIDController,
    pitch_controller: PIDController,
    yaw_controller: PIDController,
    altitude_controller: PIDController,
    state: DroneState,
}

impl FlightController {
    pub fn new() -> Self {
        Self {
            roll_controller: PIDController::new(1.0, 0.1, 0.05),
            pitch_controller: PIDController::new(1.0, 0.1, 0.05),
            yaw_controller: PIDController::new(1.0, 0.1, 0.05),
            altitude_controller: PIDController::new(1.0, 0.1, 0.05),
            state: DroneState::new(),
        }
    }

    /// Update the drone state based on sensor readings
    pub fn update_sensors(&mut self, sensors: &SensorReadings, dt: f64) {
        // Update position based on velocity
        self.state.position.0 += self.state.velocity.0 * dt;
        self.state.position.1 += self.state.velocity.1 * dt;
        self.state.position.2 += self.state.velocity.2 * dt;
        
        // Update velocity based on accelerometer
        self.state.velocity.0 += sensors.accelerometer.0 * dt;
        self.state.velocity.1 += sensors.accelerometer.1 * dt;
        self.state.velocity.2 += sensors.accelerometer.2 * dt;
        
        // Update attitude based on gyroscope
        self.state.attitude.0 += sensors.gyroscope.0 * dt;
        self.state.attitude.1 += sensors.gyroscope.1 * dt;
        self.state.attitude.2 += sensors.gyroscope.2 * dt;
        
        // Update battery level (simple drain model)
        self.state.battery_level -= 0.1 * dt;
        if self.state.battery_level < 0.0 {
            self.state.battery_level = 0.0;
        }
    }

    /// Compute control outputs based on desired state
    pub fn compute_control(&mut self, desired: &DroneState, dt: f64) -> ControlCommands {
        let roll_error = desired.attitude.0 - self.state.attitude.0;
        let pitch_error = desired.attitude.1 - self.state.attitude.1;
        let yaw_error = desired.attitude.2 - self.state.attitude.2;
        let altitude_error = desired.position.2 - self.state.position.2;
        
        let roll_output = self.roll_controller.update(roll_error, dt);
        let pitch_output = self.pitch_controller.update(pitch_error, dt);
        let yaw_output = self.yaw_controller.update(yaw_error, dt);
        let altitude_output = self.altitude_controller.update(altitude_error, dt);
        
        ControlCommands {
            throttle: altitude_output,
            roll: roll_output,
            pitch: pitch_output,
            yaw: yaw_output,
        }
    }

    /// Get the current drone state
    pub fn get_state(&self) -> &DroneState {
        &self.state
    }

    /// Arm the drone
    pub fn arm(&mut self) {
        self.state.armed = true;
        println!("Drone armed");
    }

    /// Disarm the drone
    pub fn disarm(&mut self) {
        self.state.armed = false;
        println!("Drone disarmed");
    }
}

/// Simulate sensor readings
pub fn simulate_sensor_readings() -> SensorReadings {
    let mut rng = rand::thread_rng();
    
    SensorReadings {
        accelerometer: (
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        ),
        gyroscope: (
            rng.gen_range(-0.1..0.1),
            rng.gen_range(-0.1..0.1),
            rng.gen_range(-0.1..0.1),
        ),
        magnetometer: (
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        ),
        barometer: rng.gen_range(1000.0..1020.0),
        gps: (
            rng.gen_range(-90.0..90.0),
            rng.gen_range(-180.0..180.0),
            rng.gen_range(0.0..1000.0),
        ),
    }
}

/// Real-time task scheduler simulation
pub struct RealTimeScheduler {
    tasks: Vec<Box<dyn RealTimeTask>>,
    last_update: Instant,
}

impl RealTimeScheduler {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            last_update: Instant::now(),
        }
    }

    pub fn add_task(&mut self, task: Box<dyn RealTimeTask>) {
        self.tasks.push(task);
    }

    pub fn run_cycle(&mut self) {
        let now = Instant::now();
        let dt = (now - self.last_update).as_secs_f64();
        self.last_update = now;
        
        // Execute all tasks
        for task in &mut self.tasks {
            task.execute(dt);
        }
    }
}

/// Trait for real-time tasks
pub trait RealTimeTask {
    fn execute(&mut self, dt: f64);
    fn priority(&self) -> u8;
}

/// Sensor reading task
pub struct SensorTask {
    controller: FlightController,
}

impl SensorTask {
    pub fn new(controller: FlightController) -> Self {
        Self { controller }
    }
}

impl RealTimeTask for SensorTask {
    fn execute(&mut self, dt: f64) {
        let sensors = simulate_sensor_readings();
        self.controller.update_sensors(&sensors, dt);
        println!("Sensor task executed, dt: {:.4}s", dt);
    }

    fn priority(&self) -> u8 {
        10 // High priority
    }
}

/// Control task
pub struct ControlTask {
    controller: FlightController,
    desired_state: DroneState,
}

impl ControlTask {
    pub fn new(controller: FlightController) -> Self {
        let desired_state = DroneState::new();
        Self {
            controller,
            desired_state,
        }
    }
}

impl RealTimeTask for ControlTask {
    fn execute(&mut self, dt: f64) {
        let commands = self.controller.compute_control(&self.desired_state, dt);
        println!("Control task executed: {:?}", commands);
    }

    fn priority(&self) -> u8 {
        5 // Medium priority
    }
}

/// Telemetry task
pub struct TelemetryTask {
    controller: FlightController,
}

impl TelemetryTask {
    pub fn new(controller: FlightController) -> Self {
        Self { controller }
    }
}

impl RealTimeTask for TelemetryTask {
    fn execute(&mut self, _dt: f64) {
        let state = self.controller.get_state();
        println!("Telemetry: Position({:.2}, {:.2}, {:.2}), Battery: {:.1}%", 
                 state.position.0, state.position.1, state.position.2, state.battery_level);
    }

    fn priority(&self) -> u8 {
        1 // Low priority
    }
}

/// Example of a simple real-time system
pub fn simple_realtime_example() {
    println!("Starting simple real-time system example...");
    
    let mut controller = FlightController::new();
    let mut scheduler = RealTimeScheduler::new();
    
    // Add tasks to scheduler
    scheduler.add_task(Box::new(SensorTask::new(controller)));
    scheduler.add_task(Box::new(ControlTask::new(controller)));
    scheduler.add_task(Box::new(TelemetryTask::new(controller)));
    
    // Simulate running for a few cycles
    for i in 0..10 {
        println!("Cycle {}", i);
        scheduler.run_cycle();
        std::thread::sleep(Duration::from_millis(100));
    }
}

/// Example of PID controller usage
pub fn pid_controller_example() {
    println!("Starting PID controller example...");
    
    let mut pid = PIDController::new(1.0, 0.1, 0.05);
    let mut current_value = 0.0;
    let target_value = 10.0;
    let dt = 0.1; // 100ms per update
    
    println!("Target value: {}", target_value);
    
    for i in 0..50 {
        let error = target_value - current_value;
        let output = pid.update(error, dt);
        current_value += output * dt;
        
        println!("Step {}: Error={:.3}, Output={:.3}, Value={:.3}", 
                 i, error, output, current_value);
        
        // Small delay to simulate real-time
        std::thread::sleep(Duration::from_millis(10));
    }
}

/// Example of drone state management
pub fn drone_state_example() {
    println!("Starting drone state management example...");
    
    let mut drone = DroneState::new();
    println!("Initial state: {:?}", drone);
    
    // Arm the drone
    drone.armed = true;
    println!("Armed: {}", drone.armed);
    
    // Update position
    drone.position = (10.0, 20.0, 30.0);
    println!("New position: ({}, {}, {})", 
             drone.position.0, drone.position.1, drone.position.2);
    
    // Update battery
    drone.battery_level = 75.0;
    println!("Battery level: {}%", drone.battery_level);
    
    // Disarm the drone
    drone.armed = false;
    println!("Armed: {}", drone.armed);
}

/// Example of sensor fusion
pub fn sensor_fusion_example() {
    println!("Starting sensor fusion example...");
    
    // Simulate readings from multiple sensors
    let accelerometer = (0.1, 0.2, 9.8); // m/s² (gravity)
    let gyroscope = (0.01, 0.02, 0.03);  // rad/s
    let magnetometer = (0.5, 0.3, 0.2);  // normalized
    let barometer = 1013.25;             // hPa
    let gps = (45.0, -122.0, 100.0);     // lat, lon, alt
    
    println!("Accelerometer: ({:.2}, {:.2}, {:.2}) m/s²", 
             accelerometer.0, accelerometer.1, accelerometer.2);
    println!("Gyroscope: ({:.3}, {:.3}, {:.3}) rad/s", 
             gyroscope.0, gyroscope.1, gyroscope.2);
    println!("Magnetometer: ({:.2}, {:.2}, {:.2})", 
             magnetometer.0, magnetometer.1, magnetometer.2);
    println!("Barometer: {:.2} hPa", barometer);
    println!("GPS: ({:.4}, {:.4}, {:.1}m)", gps.0, gps.1, gps.2);
    
    // Simple sensor fusion example
    let fused_altitude = gps.2; // Use GPS altitude as primary
    let fused_heading = f64::atan2(magnetometer.1, magnetometer.0).to_degrees();
    
    println!("Fused altitude: {:.1}m", fused_altitude);
    println!("Fused heading: {:.1}°", fused_heading);
}

/// Example of hard real-time constraints
pub fn hard_realtime_example() {
    println!("Starting hard real-time constraints example...");
    
    let start_time = Instant::now();
    let deadline = Duration::from_millis(10); // 10ms deadline
    
    for i in 0..5 {
        let task_start = Instant::now();
        
        // Simulate some work
        let work_time = Duration::from_millis(rand::random::<u64>() % 15);
        std::thread::sleep(work_time);
        
        let task_duration = task_start.elapsed();
        
        if task_duration <= deadline {
            println!("Task {}: Completed in {:?} (within {:?} deadline)", 
                     i, task_duration, deadline);
        } else {
            println!("Task {}: COMPLETED in {:?} (MISSED {:?} deadline)", 
                     i, task_duration, deadline);
        }
    }
    
    let total_duration = start_time.elapsed();
    println!("Total execution time: {:?}", total_duration);
}

/// Example usage of real-time and embedded functions
pub fn example_usage() {
    println!("Real-Time and Embedded Examples");
    println!("=============================");
    
    println!("\n1. Simple real-time system example:");
    simple_realtime_example();
    
    println!("\n2. PID controller example:");
    pid_controller_example();
    
    println!("\n3. Drone state management example:");
    drone_state_example();
    
    println!("\n4. Sensor fusion example:");
    sensor_fusion_example();
    
    println!("\n5. Hard real-time constraints example:");
    hard_realtime_example();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drone_state() {
        let mut drone = DroneState::new();
        
        assert_eq!(drone.position, (0.0, 0.0, 0.0));
        assert_eq!(drone.velocity, (0.0, 0.0, 0.0));
        assert_eq!(drone.attitude, (0.0, 0.0, 0.0));
        assert_eq!(drone.battery_level, 100.0);
        assert_eq!(drone.armed, false);
        
        drone.armed = true;
        assert_eq!(drone.armed, true);
    }

    #[test]
    fn test_pid_controller() {
        let mut pid = PIDController::new(1.0, 0.1, 0.05);
        
        let error = 5.0;
        let dt = 0.1;
        let output = pid.update(error, dt);
        
        assert!(output > 0.0);
        
        // Test reset
        pid.reset();
        assert_eq!(pid.previous_error, 0.0);
        assert_eq!(pid.integral, 0.0);
    }

    #[test]
    fn test_flight_controller() {
        let mut controller = FlightController::new();
        let state = controller.get_state();
        
        assert_eq!(state.position, (0.0, 0.0, 0.0));
        assert_eq!(state.battery_level, 100.0);
        
        controller.arm();
        assert_eq!(controller.get_state().armed, true);
        
        controller.disarm();
        assert_eq!(controller.get_state().armed, false);
    }

    #[test]
    fn test_sensor_readings() {
        let sensors = simulate_sensor_readings();
        
        // Just verify the structure is created
        assert!(sensors.accelerometer.0.is_finite());
        assert!(sensors.gyroscope.0.is_finite());
        assert!(sensors.magnetometer.0.is_finite());
        assert!(sensors.barometer.is_finite());
        assert!(sensors.gps.0.is_finite());
    }
}
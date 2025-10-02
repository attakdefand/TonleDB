//! State machines and manual futures examples
//!
//! This module demonstrates how to implement a custom binary protocol
//! by hand-coding the Future trait as a state machine.

use futures::future::Future;
use futures::task::{Context, Poll};
use pin_project::pin_project;
use std::pin::Pin;
use std::time::{Duration, Instant};
use tokio::time::{sleep, Sleep};

/// States for a simple TCP connection state machine
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Disconnecting,
}

/// A simple TCP connection simulator
pub struct TcpConnection {
    pub state: ConnectionState,
    pub address: String,
    connect_time: Option<Instant>,
}

impl TcpConnection {
    pub fn new(address: String) -> Self {
        Self {
            state: ConnectionState::Disconnected,
            address,
            connect_time: None,
        }
    }

    /// Start connecting to the remote address
    pub fn connect(&mut self) {
        if self.state == ConnectionState::Disconnected {
            self.state = ConnectionState::Connecting;
            self.connect_time = Some(Instant::now());
            println!("Connecting to {}", self.address);
        }
    }

    /// Complete the connection
    pub fn connected(&mut self) {
        if self.state == ConnectionState::Connecting {
            self.state = ConnectionState::Connected;
            println!("Connected to {}", self.address);
        }
    }

    /// Start disconnecting
    pub fn disconnect(&mut self) {
        if self.state == ConnectionState::Connected {
            self.state = ConnectionState::Disconnecting;
            println!("Disconnecting from {}", self.address);
        }
    }

    /// Complete the disconnection
    pub fn disconnected(&mut self) {
        if self.state == ConnectionState::Disconnecting {
            self.state = ConnectionState::Disconnected;
            self.connect_time = None;
            println!("Disconnected from {}", self.address);
        }
    }

    /// Get connection duration
    pub fn connection_duration(&self) -> Option<Duration> {
        self.connect_time.map(|start| start.elapsed())
    }
}

/// A state machine for a simple protocol that sends and receives messages
#[derive(Debug, Clone, PartialEq)]
pub enum ProtocolState {
    Idle,
    Sending { message_id: u32 },
    WaitingForResponse { message_id: u32, sent_at: Instant },
    Received { message_id: u32, response: String },
    Error { message_id: u32, error: String },
}

/// A simple protocol handler
pub struct ProtocolHandler {
    pub state: ProtocolState,
    pub peer_address: String,
}

impl ProtocolHandler {
    pub fn new(peer_address: String) -> Self {
        Self {
            state: ProtocolState::Idle,
            peer_address,
        }
    }

    /// Send a message
    pub fn send_message(&mut self, message_id: u32, _content: &str) {
        if let ProtocolState::Idle = self.state {
            self.state = ProtocolState::Sending { message_id };
            println!("Sending message {} to {}", message_id, self.peer_address);
        }
    }

    /// Mark message as sent
    pub fn message_sent(&mut self, message_id: u32) {
        if let ProtocolState::Sending { message_id: current_id } = self.state {
            if current_id == message_id {
                self.state = ProtocolState::WaitingForResponse {
                    message_id,
                    sent_at: Instant::now(),
                };
                println!("Message {} sent, waiting for response", message_id);
            }
        }
    }

    /// Receive a response
    pub fn receive_response(&mut self, message_id: u32, response: String) {
        match &self.state {
            ProtocolState::WaitingForResponse { message_id: current_id, .. } => {
                if *current_id == message_id {
                    self.state = ProtocolState::Received {
                        message_id,
                        response,
                    };
                    println!("Received response for message {}: {}", message_id, response);
                }
            }
            _ => {
                self.state = ProtocolState::Error {
                    message_id,
                    error: "Unexpected response".to_string(),
                };
                println!("Error: Unexpected response for message {}", message_id);
            }
        }
    }

    /// Handle timeout
    pub fn handle_timeout(&mut self, message_id: u32) {
        if let ProtocolState::WaitingForResponse { message_id: current_id, .. } = self.state {
            if current_id == message_id {
                self.state = ProtocolState::Error {
                    message_id,
                    error: "Timeout".to_string(),
                };
                println!("Timeout for message {}", message_id);
            }
        }
    }

    /// Reset to idle state
    pub fn reset(&mut self) {
        self.state = ProtocolState::Idle;
    }
}

/// A custom Future that implements a simple timer
#[pin_project]
pub struct TimerFuture {
    #[pin]
    sleep: Sleep,
    duration: Duration,
    started: Instant,
}

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        Self {
            sleep: sleep(duration),
            duration,
            started: Instant::now(),
        }
    }
}

impl Future for TimerFuture {
    type Output = Duration;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        
        match this.sleep.poll(cx) {
            Poll::Ready(()) => {
                let elapsed = this.started.elapsed();
                Poll::Ready(elapsed)
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

/// A custom Future that implements a simple state machine for a download process
#[pin_project]
pub struct DownloadFuture {
    state: DownloadState,
    #[pin]
    sleep: Option<Sleep>,
    progress: u32,
    total: u32,
}

#[derive(Debug, Clone, PartialEq)]
enum DownloadState {
    Initializing,
    Downloading,
    Completed,
    Failed(String),
}

impl DownloadFuture {
    pub fn new(total: u32) -> Self {
        Self {
            state: DownloadState::Initializing,
            sleep: Some(sleep(Duration::from_millis(100))),
            progress: 0,
            total,
        }
    }
}

impl Future for DownloadFuture {
    type Output = Result<u32, String>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();
        
        match &this.state {
            DownloadState::Initializing => {
                println!("Initializing download...");
                *this.state = DownloadState::Downloading;
                // Poll again immediately
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            DownloadState::Downloading => {
                // Check if we're done
                if *this.progress >= *this.total {
                    *this.state = DownloadState::Completed;
                    cx.waker().wake_by_ref();
                    return Poll::Pending;
                }
                
                // Check if sleep is ready
                if let Some(sleep) = this.sleep.as_mut().as_pin_mut() {
                    match sleep.poll(cx) {
                        Poll::Ready(()) => {
                            // Simulate progress
                            *this.progress += 1;
                            println!("Download progress: {}/{}", this.progress, this.total);
                            
                            // Reset sleep for next iteration
                            this.sleep.set(Some(sleep(Duration::from_millis(100))));
                            
                            // Wake up immediately to continue
                            cx.waker().wake_by_ref();
                            Poll::Pending
                        }
                        Poll::Pending => Poll::Pending,
                    }
                } else {
                    Poll::Pending
                }
            }
            DownloadState::Completed => {
                Poll::Ready(Ok(*this.progress))
            }
            DownloadState::Failed(error) => {
                Poll::Ready(Err(error.clone()))
            }
        }
    }
}

/// A custom Future that implements a simple state machine for a network request
#[pin_project]
pub struct NetworkRequestFuture {
    state: RequestState,
    #[pin]
    timeout: Sleep,
    started: Instant,
}

#[derive(Debug, Clone, PartialEq)]
enum RequestState {
    Sending,
    Waiting,
    Completed(String),
    TimedOut,
    Error(String),
}

impl NetworkRequestFuture {
    pub fn new() -> Self {
        Self {
            state: RequestState::Sending,
            timeout: sleep(Duration::from_secs(5)),
            started: Instant::now(),
        }
    }
}

impl Future for NetworkRequestFuture {
    type Output = Result<String, String>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();
        
        match &this.state {
            RequestState::Sending => {
                println!("Sending network request...");
                *this.state = RequestState::Waiting;
                // Simulate immediate send, wake up to wait for response
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            RequestState::Waiting => {
                // Check if timeout has expired
                match this.timeout.as_mut().poll(cx) {
                    Poll::Ready(()) => {
                        *this.state = RequestState::TimedOut;
                        cx.waker().wake_by_ref();
                        Poll::Pending
                    }
                    Poll::Pending => {
                        // Simulate random completion
                        if this.started.elapsed() > Duration::from_millis(2000) && rand::random::<u8>() < 50 {
                            *this.state = RequestState::Completed("Success response".to_string());
                            cx.waker().wake_by_ref();
                            Poll::Pending
                        } else {
                            // Still waiting
                            Poll::Pending
                        }
                    }
                }
            }
            RequestState::Completed(response) => {
                Poll::Ready(Ok(response.clone()))
            }
            RequestState::TimedOut => {
                Poll::Ready(Err("Request timed out".to_string()))
            }
            RequestState::Error(error) => {
                Poll::Ready(Err(error.clone()))
            }
        }
    }
}

/// Example of using the TCP connection state machine
pub fn tcp_connection_example() {
    println!("Starting TCP connection state machine example...");
    
    let mut connection = TcpConnection::new("192.168.1.1:8080".to_string());
    
    println!("Initial state: {:?}", connection.state);
    
    connection.connect();
    println!("After connect: {:?}", connection.state);
    
    connection.connected();
    println!("After connected: {:?}", connection.state);
    
    if let Some(duration) = connection.connection_duration() {
        println!("Connected for: {:?}", duration);
    }
    
    connection.disconnect();
    println!("After disconnect: {:?}", connection.state);
    
    connection.disconnected();
    println!("After disconnected: {:?}", connection.state);
}

/// Example of using the protocol handler state machine
pub fn protocol_handler_example() {
    println!("Starting protocol handler state machine example...");
    
    let mut handler = ProtocolHandler::new("192.168.1.1:9090".to_string());
    
    println!("Initial state: {:?}", handler.state);
    
    handler.send_message(1, "Hello");
    println!("After send_message: {:?}", handler.state);
    
    handler.message_sent(1);
    println!("After message_sent: {:?}", handler.state);
    
    handler.receive_response(1, "Hello, World!".to_string());
    println!("After receive_response: {:?}", handler.state);
    
    handler.reset();
    println!("After reset: {:?}", handler.state);
}

/// Example of using the custom timer future
pub async fn timer_future_example() {
    println!("Starting timer future example...");
    
    let timer = TimerFuture::new(Duration::from_millis(500));
    let elapsed = timer.await;
    
    println!("Timer completed in {:?}", elapsed);
}

/// Example of using the download future
pub async fn download_future_example() {
    println!("Starting download future example...");
    
    let download = DownloadFuture::new(10);
    match download.await {
        Ok(progress) => println!("Download completed with progress: {}", progress),
        Err(error) => println!("Download failed: {}", error),
    }
}

/// Example of using the network request future
pub async fn network_request_future_example() {
    println!("Starting network request future example...");
    
    let request = NetworkRequestFuture::new();
    match request.await {
        Ok(response) => println!("Request succeeded: {}", response),
        Err(error) => println!("Request failed: {}", error),
    }
}

/// Example of manually implementing a Future for a simple counter
pub struct CounterFuture {
    count: u32,
    target: u32,
}

impl CounterFuture {
    pub fn new(target: u32) -> Self {
        Self { count: 0, target }
    }
}

impl Future for CounterFuture {
    type Output = u32;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.count >= self.target {
            Poll::Ready(self.count)
        } else {
            self.count += 1;
            // Wake up immediately to continue counting
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

/// Example of using the counter future
pub async fn counter_future_example() {
    println!("Starting counter future example...");
    
    let counter = CounterFuture::new(5);
    let result = counter.await;
    
    println!("Counter reached: {}", result);
}

/// Example usage of state machines and manual futures functions
pub fn example_usage() {
    println!("State Machines and Manual Futures Examples");
    println!("========================================");
    
    println!("\n1. TCP connection state machine:");
    tcp_connection_example();
    
    println!("\n2. Protocol handler state machine:");
    protocol_handler_example();
    
    println!("\n3. Timer future example:");
    // Note: This would need to be called in an async context
    println!("   Call timer_future_example().await to see this in action");
    
    println!("\n4. Download future example:");
    println!("   Call download_future_example().await to see this in action");
    
    println!("\n5. Network request future example:");
    println!("   Call network_request_future_example().await to see this in action");
    
    println!("\n6. Counter future example:");
    println!("   Call counter_future_example().await to see this in action");
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor::block_on;

    #[test]
    fn test_tcp_connection_state_machine() {
        let mut connection = TcpConnection::new("127.0.0.1:8080".to_string());
        
        assert_eq!(connection.state, ConnectionState::Disconnected);
        
        connection.connect();
        assert_eq!(connection.state, ConnectionState::Connecting);
        
        connection.connected();
        assert_eq!(connection.state, ConnectionState::Connected);
        
        connection.disconnect();
        assert_eq!(connection.state, ConnectionState::Disconnecting);
        
        connection.disconnected();
        assert_eq!(connection.state, ConnectionState::Disconnected);
    }

    #[test]
    fn test_protocol_handler_state_machine() {
        let mut handler = ProtocolHandler::new("127.0.0.1:9090".to_string());
        
        assert_eq!(handler.state, ProtocolState::Idle);
        
        handler.send_message(1, "test");
        assert_eq!(handler.state, ProtocolState::Sending { message_id: 1 });
        
        handler.message_sent(1);
        match handler.state {
            ProtocolState::WaitingForResponse { message_id, .. } => {
                assert_eq!(message_id, 1);
            }
            _ => panic!("Expected WaitingForResponse state"),
        }
    }

    #[test]
    fn test_timer_future() {
        let timer = TimerFuture::new(Duration::from_millis(10));
        let elapsed = block_on(timer);
        assert!(elapsed >= Duration::from_millis(10));
    }

    #[test]
    fn test_counter_future() {
        let counter = CounterFuture::new(3);
        let result = block_on(counter);
        assert_eq!(result, 3);
    }
}
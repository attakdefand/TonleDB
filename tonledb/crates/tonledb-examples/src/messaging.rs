//! Message-passing and actor examples
//!
//! This module demonstrates how to use actix actors to route messages
//! between connected clients in a real-time chat server.

use actix::prelude::*;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// A message representing a chat message
#[derive(Message)]
#[rtype(result = "()")]
pub struct ChatMessage {
    pub sender: String,
    pub content: String,
    pub timestamp: Instant,
}

/// A message to register a client
#[derive(Message)]
#[rtype(result = "ClientId")]
pub struct RegisterClient {
    pub addr: Recipient<ChatMessage>,
}

/// A message to unregister a client
#[derive(Message)]
#[rtype(result = "()")]
pub struct UnregisterClient {
    pub id: ClientId,
}

/// A message to send a chat message to all clients
#[derive(Message)]
#[rtype(result = "()")]
pub struct BroadcastMessage {
    pub sender: String,
    pub content: String,
}

/// A unique identifier for a client
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClientId(u32);

/// A chat server actor
pub struct ChatServer {
    clients: HashMap<ClientId, Recipient<ChatMessage>>,
    next_id: u32,
}

impl ChatServer {
    /// Create a new chat server
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
            next_id: 1,
        }
    }
}

impl Actor for ChatServer {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("Chat server started");
    }
}

/// Handler for RegisterClient messages
impl Handler<RegisterClient> for ChatServer {
    type Result = ClientId;

    fn handle(&mut self, msg: RegisterClient, _ctx: &mut Self::Context) -> Self::Result {
        let id = ClientId(self.next_id);
        self.next_id += 1;
        
        self.clients.insert(id, msg.addr);
        println!("Client {:?} registered", id);
        
        id
    }
}

/// Handler for UnregisterClient messages
impl Handler<UnregisterClient> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: UnregisterClient, _ctx: &mut Self::Context) -> Self::Result {
        self.clients.remove(&msg.id);
        println!("Client {:?} unregistered", msg.id);
    }
}

/// Handler for BroadcastMessage messages
impl Handler<BroadcastMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: BroadcastMessage, _ctx: &mut Self::Context) -> Self::Result {
        let chat_msg = ChatMessage {
            sender: msg.sender,
            content: msg.content,
            timestamp: Instant::now(),
        };

        // Send the message to all clients
        for (_id, client) in &self.clients {
            let _ = client.do_send(chat_msg.clone());
        }
    }
}

/// A chat client actor
pub struct ChatClient {
    name: String,
    server: Addr<ChatServer>,
    id: Option<ClientId>,
}

impl ChatClient {
    /// Create a new chat client
    pub fn new(name: String, server: Addr<ChatServer>) -> Self {
        Self {
            name,
            server,
            id: None,
        }
    }
}

impl Actor for ChatClient {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // Register with the server
        let addr = ctx.address().recipient();
        let server = self.server.clone();
        
        let fut = async move {
            server.send(RegisterClient { addr }).await
        }
        .into_actor(self)
        .then(|res, act, _ctx| {
            if let Ok(id) = res {
                act.id = Some(id);
                println!("{} registered with ID {:?}", act.name, id);
            }
            async {}.into_actor(act)
        });
        
        ctx.spawn(fut);
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        // Unregister from the server
        if let Some(id) = self.id {
            let server = self.server.clone();
            Arbiter::spawn(async move {
                server.send(UnregisterClient { id }).await.ok();
            });
        }
        Running::Stop
    }
}

/// Handler for ChatMessage messages
impl Handler<ChatMessage> for ChatClient {
    type Result = ();

    fn handle(&mut self, msg: ChatMessage, _ctx: &mut Self::Context) -> Self::Result {
        println!("{} received message from {}: {}", 
                 self.name, msg.sender, msg.content);
    }
}

/// Example of sending messages between actors
pub fn actor_messaging_example() {
    println!("Starting actor messaging example...");
    
    // Start the actor system
    System::new().block_on(async {
        // Create a chat server
        let server = ChatServer::new().start();
        
        // Create some chat clients
        let _alice = ChatClient::new("Alice".to_string(), server.clone()).start();
        let _bob = ChatClient::new("Bob".to_string(), server.clone()).start();
        let _charlie = ChatClient::new("Charlie".to_string(), server.clone()).start();
        
        // Send a broadcast message
        server.do_send(BroadcastMessage {
            sender: "System".to_string(),
            content: "Welcome to the chat server!".to_string(),
        });
        
        // Give some time for messages to be processed
        tokio::time::sleep(Duration::from_millis(100)).await;
    });
    
    println!("Actor messaging example completed");
}

/// A simple actor that performs calculations
#[derive(Message)]
#[rtype(result = "i32")]
pub struct Calculate {
    pub a: i32,
    pub b: i32,
    pub operation: Operation,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct PrintResult {
    pub value: i32,
}

pub enum Operation {
    Add,
    Multiply,
}

pub struct CalculatorActor;

impl Actor for CalculatorActor {
    type Context = Context<Self>;
}

impl Handler<Calculate> for CalculatorActor {
    type Result = i32;

    fn handle(&mut self, msg: Calculate, _ctx: &mut Self::Context) -> Self::Result {
        match msg.operation {
            Operation::Add => msg.a + msg.b,
            Operation::Multiply => msg.a * msg.b,
        }
    }
}

impl Handler<PrintResult> for CalculatorActor {
    type Result = ();

    fn handle(&mut self, msg: PrintResult, _ctx: &mut Self::Context) -> Self::Result {
        println!("Calculation result: {}", msg.value);
    }
}

/// Example of actor-based calculations
pub fn actor_calculation_example() {
    println!("Starting actor calculation example...");
    
    System::new().block_on(async {
        let calculator = CalculatorActor.start();
        
        // Send calculation requests
        let result1 = calculator.send(Calculate {
            a: 10,
            b: 5,
            operation: Operation::Add,
        }).await.unwrap();
        
        let result2 = calculator.send(Calculate {
            a: 10,
            b: 5,
            operation: Operation::Multiply,
        }).await.unwrap();
        
        println!("10 + 5 = {}", result1);
        println!("10 * 5 = {}", result2);
        
        // Send result to be printed
        calculator.do_send(PrintResult { value: result1 + result2 });
        
        // Give some time for messages to be processed
        tokio::time::sleep(Duration::from_millis(50)).await;
    });
    
    println!("Actor calculation example completed");
}

/// Example usage of message-passing and actor functions
pub fn example_usage() {
    println!("Message-Passing and Actor Examples");
    println!("=================================");
    
    println!("\n1. Actor messaging example:");
    actor_messaging_example();
    
    println!("\n2. Actor calculation example:");
    actor_calculation_example();
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_rt::System;

    #[test]
    fn test_chat_server_actor() {
        System::new().block_on(async {
            let server = ChatServer::new().start();
            
            // Test registering a client
            let client_addr = actix::spawn(async {}).into_actor::<()>().map(|_, _, _| ()).into_recipient();
            let client_id = server.send(RegisterClient { addr: client_addr }).await.unwrap();
            
            // Test unregistering a client
            server.send(UnregisterClient { id: client_id }).await.unwrap();
        });
    }

    #[test]
    fn test_calculator_actor() {
        System::new().block_on(async {
            let calculator = CalculatorActor.start();
            
            // Test addition
            let result = calculator.send(Calculate {
                a: 2,
                b: 3,
                operation: Operation::Add,
            }).await.unwrap();
            assert_eq!(result, 5);
            
            // Test multiplication
            let result = calculator.send(Calculate {
                a: 2,
                b: 3,
                operation: Operation::Multiply,
            }).await.unwrap();
            assert_eq!(result, 6);
        });
    }
}
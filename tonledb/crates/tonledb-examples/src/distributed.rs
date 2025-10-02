//! Distributed systems examples
//!
//! This module demonstrates microservices exchanging protobuf messages
//! over gRPC with tonic, plus service discovery.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tonic::{transport::Server, Request, Response, Status};

/// A simple user entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
}

/// A simple product entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub price: f64,
}

/// A simple order entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: u32,
    pub user_id: u32,
    pub product_id: u32,
    pub quantity: u32,
    pub total: f64,
}

/// In-memory storage for our microservices
pub struct ServiceStorage {
    users: Arc<RwLock<HashMap<u32, User>>>,
    products: Arc<RwLock<HashMap<u32, Product>>>,
    orders: Arc<RwLock<HashMap<u32, Order>>>,
}

impl ServiceStorage {
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
            products: Arc::new(RwLock::new(HashMap::new())),
            orders: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

/// User service implementation
pub struct UserService {
    storage: Arc<ServiceStorage>,
}

#[tonic::async_trait]
impl user_service_server::UserService for UserService {
    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<GetUserResponse>, Status> {
        let user_id = request.into_inner().user_id;
        
        let users = self.storage.users.read().await;
        if let Some(user) = users.get(&user_id) {
            Ok(Response::new(GetUserResponse {
                user: Some(UserProto {
                    id: user.id,
                    name: user.name.clone(),
                    email: user.email.clone(),
                }),
            }))
        } else {
            Err(Status::not_found("User not found"))
        }
    }

    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, Status> {
        let req = request.into_inner();
        
        let user = User {
            id: req.id,
            name: req.name.clone(),
            email: req.email.clone(),
        };
        
        let mut users = self.storage.users.write().await;
        users.insert(user.id, user.clone());
        
        Ok(Response::new(CreateUserResponse {
            user: Some(UserProto {
                id: user.id,
                name: user.name,
                email: user.email,
            }),
        }))
    }
}

/// Product service implementation
pub struct ProductService {
    storage: Arc<ServiceStorage>,
}

#[tonic::async_trait]
impl product_service_server::ProductService for ProductService {
    async fn get_product(
        &self,
        request: Request<GetProductRequest>,
    ) -> Result<Response<GetProductResponse>, Status> {
        let product_id = request.into_inner().product_id;
        
        let products = self.storage.products.read().await;
        if let Some(product) = products.get(&product_id) {
            Ok(Response::new(GetProductResponse {
                product: Some(ProductProto {
                    id: product.id,
                    name: product.name.clone(),
                    price: product.price,
                }),
            }))
        } else {
            Err(Status::not_found("Product not found"))
        }
    }

    async fn create_product(
        &self,
        request: Request<CreateProductRequest>,
    ) -> Result<Response<CreateProductResponse>, Status> {
        let req = request.into_inner();
        
        let product = Product {
            id: req.id,
            name: req.name.clone(),
            price: req.price,
        };
        
        let mut products = self.storage.products.write().await;
        products.insert(product.id, product.clone());
        
        Ok(Response::new(CreateProductResponse {
            product: Some(ProductProto {
                id: product.id,
                name: product.name,
                price: product.price,
            }),
        }))
    }
}

/// Order service implementation
pub struct OrderService {
    storage: Arc<ServiceStorage>,
}

#[tonic::async_trait]
impl order_service_server::OrderService for OrderService {
    async fn get_order(
        &self,
        request: Request<GetOrderRequest>,
    ) -> Result<Response<GetOrderResponse>, Status> {
        let order_id = request.into_inner().order_id;
        
        let orders = self.storage.orders.read().await;
        if let Some(order) = orders.get(&order_id) {
            Ok(Response::new(GetOrderResponse {
                order: Some(OrderProto {
                    id: order.id,
                    user_id: order.user_id,
                    product_id: order.product_id,
                    quantity: order.quantity,
                    total: order.total,
                }),
            }))
        } else {
            Err(Status::not_found("Order not found"))
        }
    }

    async fn create_order(
        &self,
        request: Request<CreateOrderRequest>,
    ) -> Result<Response<CreateOrderResponse>, Status> {
        let req = request.into_inner();
        
        let order = Order {
            id: req.id,
            user_id: req.user_id,
            product_id: req.product_id,
            quantity: req.quantity,
            total: req.total,
        };
        
        let mut orders = self.storage.orders.write().await;
        orders.insert(order.id, order.clone());
        
        Ok(Response::new(CreateOrderResponse {
            order: Some(OrderProto {
                id: order.id,
                user_id: order.user_id,
                product_id: order.product_id,
                quantity: order.quantity,
                total: order.total,
            }),
        }))
    }
}

/// Service discovery implementation
pub struct ServiceRegistry {
    services: Arc<RwLock<HashMap<String, String>>>, // service_name -> address
}

impl ServiceRegistry {
    pub fn new() -> Self {
        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register_service(&self, name: String, address: String) {
        let mut services = self.services.write().await;
        services.insert(name, address);
    }

    pub async fn get_service(&self, name: &str) -> Option<String> {
        let services = self.services.read().await;
        services.get(name).cloned()
    }

    pub async fn list_services(&self) -> Vec<(String, String)> {
        let services = self.services.read().await;
        services.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }
}

/// Example of a simple gRPC client
pub async fn grpc_client_example() {
    println!("Starting gRPC client example...");
    
    // In a real implementation, you would connect to a gRPC server
    println!("To connect to a gRPC service, you would use:");
    println!("let channel = tonic::transport::Channel::connect(\"http://[::1]:50051\").await?;");
    println!("let mut client = your_service_client::YourServiceClient::new(channel);");
    
    // Then you would call methods on the client
    println!("let request = tonic::Request::new(YourRequest {{ ... }});");
    println!("let response = client.your_method(request).await?;");
}

/// Example of service discovery
pub async fn service_discovery_example() {
    println!("Starting service discovery example...");
    
    let registry = ServiceRegistry::new();
    
    // Register some services
    registry.register_service("user-service".to_string(), "127.0.0.1:50051".to_string()).await;
    registry.register_service("product-service".to_string(), "127.0.0.1:50052".to_string()).await;
    registry.register_service("order-service".to_string(), "127.0.0.1:50053".to_string()).await;
    
    // List registered services
    let services = registry.list_services().await;
    println!("Registered services:");
    for (name, address) in services {
        println!("  {}: {}", name, address);
    }
    
    // Look up a specific service
    if let Some(address) = registry.get_service("user-service").await {
        println!("Found user-service at: {}", address);
    } else {
        println!("user-service not found");
    }
}

/// Example of microservice communication
pub async fn microservice_communication_example() {
    println!("Starting microservice communication example...");
    
    let storage = Arc::new(ServiceStorage::new());
    let registry = ServiceRegistry::new();
    
    // Initialize some data
    {
        let mut users = storage.users.write().await;
        users.insert(1, User {
            id: 1,
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        });
        users.insert(2, User {
            id: 2,
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
        });
    }
    
    {
        let mut products = storage.products.write().await;
        products.insert(1, Product {
            id: 1,
            name: "Rust Book".to_string(),
            price: 39.99,
        });
        products.insert(2, Product {
            id: 2,
            name: "T-Shirt".to_string(),
            price: 19.99,
        });
    }
    
    // Simulate service calls
    let users = storage.users.read().await;
    if let Some(user) = users.get(&1) {
        println!("Retrieved user: {} ({})", user.name, user.email);
    }
    
    let products = storage.products.read().await;
    if let Some(product) = products.get(&1) {
        println!("Retrieved product: {} (${:.2})", product.name, product.price);
    }
    
    // Create an order
    let order = Order {
        id: 1,
        user_id: 1,
        product_id: 1,
        quantity: 2,
        total: 79.98,
    };
    
    let mut orders = storage.orders.write().await;
    orders.insert(order.id, order.clone());
    println!("Created order #{} for user {} product {} quantity {} total ${:.2}", 
             order.id, order.user_id, order.product_id, order.quantity, order.total);
}

/// Example of load balancing between services
pub fn load_balancing_example() {
    println!("Starting load balancing example...");
    
    // Simulate multiple instances of a service
    let service_instances = vec![
        "127.0.0.1:50051",
        "127.0.0.1:50052",
        "127.0.0.1:50053",
    ];
    
    // Simple round-robin load balancing
    let mut current_index = 0;
    
    for i in 1..=10 {
        let instance = service_instances[current_index];
        println!("Request {}: Routed to {}", i, instance);
        current_index = (current_index + 1) % service_instances.len();
    }
}

/// Example of circuit breaker pattern
pub fn circuit_breaker_example() {
    println!("Starting circuit breaker example...");
    
    // Simulate service calls with failure tracking
    let mut failure_count = 0;
    const FAILURE_THRESHOLD: u32 = 3;
    let mut circuit_open = false;
    
    for i in 1..=10 {
        if circuit_open {
            println!("Request {}: Circuit breaker is OPEN - failing fast", i);
            continue;
        }
        
        // Simulate service call
        let success = rand::random::<bool>();
        
        if success {
            println!("Request {}: SUCCESS", i);
            failure_count = 0; // Reset failure count on success
        } else {
            println!("Request {}: FAILED", i);
            failure_count += 1;
            
            if failure_count >= FAILURE_THRESHOLD {
                circuit_open = true;
                println!("  Circuit breaker OPENED after {} failures", failure_count);
            }
        }
    }
}

/// Example usage of distributed systems functions
pub fn example_usage() {
    println!("Distributed Systems Examples");
    println!("==========================");
    
    println!("\n1. gRPC client example:");
    // Note: This would need to be called in an async context
    println!("   Call grpc_client_example().await to see this in action");
    
    println!("\n2. Service discovery example:");
    println!("   Call service_discovery_example().await to see this in action");
    
    println!("\n3. Microservice communication example:");
    println!("   Call microservice_communication_example().await to see this in action");
    
    println!("\n4. Load balancing example:");
    load_balancing_example();
    
    println!("\n5. Circuit breaker example:");
    circuit_breaker_example();
}

// gRPC service definitions (these would normally be generated from .proto files)
tonic::include_proto!("user_service");
tonic::include_proto!("product_service");
tonic::include_proto!("order_service");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_entity() {
        let user = User {
            id: 1,
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        };
        
        assert_eq!(user.id, 1);
        assert_eq!(user.name, "Alice");
        assert_eq!(user.email, "alice@example.com");
    }

    #[test]
    fn test_product_entity() {
        let product = Product {
            id: 1,
            name: "Rust Book".to_string(),
            price: 39.99,
        };
        
        assert_eq!(product.id, 1);
        assert_eq!(product.name, "Rust Book");
        assert_eq!(product.price, 39.99);
    }

    #[test]
    fn test_order_entity() {
        let order = Order {
            id: 1,
            user_id: 1,
            product_id: 1,
            quantity: 2,
            total: 79.98,
        };
        
        assert_eq!(order.id, 1);
        assert_eq!(order.user_id, 1);
        assert_eq!(order.product_id, 1);
        assert_eq!(order.quantity, 2);
        assert_eq!(order.total, 79.98);
    }

    #[tokio::test]
    async fn test_service_storage() {
        let storage = ServiceStorage::new();
        
        // Test user storage
        {
            let mut users = storage.users.write().await;
            users.insert(1, User {
                id: 1,
                name: "Alice".to_string(),
                email: "alice@example.com".to_string(),
            });
        }
        
        {
            let users = storage.users.read().await;
            assert!(users.contains_key(&1));
            assert_eq!(users.get(&1).unwrap().name, "Alice");
        }
    }

    #[tokio::test]
    async fn test_service_registry() {
        let registry = ServiceRegistry::new();
        
        registry.register_service("test-service".to_string(), "127.0.0.1:8080".to_string()).await;
        
        let services = registry.list_services().await;
        assert_eq!(services.len(), 1);
        assert_eq!(services[0].0, "test-service");
        assert_eq!(services[0].1, "127.0.0.1:8080");
        
        let address = registry.get_service("test-service").await;
        assert_eq!(address, Some("127.0.0.1:8080".to_string()));
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Compile the proto files
    tonic_build::compile_protos("proto/user_service.proto")?;
    tonic_build::compile_protos("proto/product_service.proto")?;
    tonic_build::compile_protos("proto/order_service.proto")?;
    
    Ok(())
}
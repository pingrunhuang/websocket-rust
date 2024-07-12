//this annotation transform async fn main() into fn main() that initialize a runtime and execute async main
#[tokio::main]
async fn main(){
    println!("hello world")
}
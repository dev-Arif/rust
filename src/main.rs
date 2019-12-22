
#[derive(Debug)]
struct User {
  name: String,
  email: String,
  age: u8
}

fn new_user(name: String, email: String, age: u8) -> User {
    let user = User {
        name : name,
        email : email,
        age : age
    };
    return user
}

fn main() {

    let a_user = User {
        name: "Arifullah Khan".to_string(),
        email: "arif@email.com".to_string(),
        age: 33
    };

    println!("{:#?}", a_user);
    println!("Name: {}", a_user.name);
    println!("Email: {}", a_user.email);
    println!("Age: {}", a_user.age);

    let another_user = User{
        name: a_user.name,
        email: a_user.email,
        age: a_user.age
    };
    println!("{:#?}", another_user);

    let name = "Baryal Khan".to_string();
    let new_user = new_user(name, "someEmail@gmail.com".to_string(), 4);
    println!("{:#?}", new_user);
    
}

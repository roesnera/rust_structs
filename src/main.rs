// println! uses a reference to data, not the data itself
// this means that references can be passed to println! then used later
// dbg! prints out data but is passed ownership unless otherwise specified as &(reference)

fn main() {
    let my_user = User {
        name: String::from("Adam"),
        email: String::from("email@example.com"),
        is_active: true,
        login_count: 1
    };

    let name = &my_user.name;
    println!("{}", name);

    // update syntax allows us to avoid unnecessary rewriting of struct fields we do not want to
    // update
    // similar to spread syntax in js

    let second_user = User {
        name: String::from("Mariah"),
        ..my_user
    };

    println!("{}", second_user.name);

    // this println! macro uses the :? to tell println to display the second_user
    // using the debug default formatting
    println!("{:?}", second_user);

    // this uses a debug format that displays each field on its own line
    println!("{:#?}", second_user);


    println!("{}", second_user.describe());

    // note that because we did not change the email field,
    // my_user is no longer valid as a whole
    // this is because we transferred ownership of the email field to second_user
    // when using update syntax, references used in the previous instance of the struct will be destroyed if any
    // fields that do not implement the Copy() trait (i.e. non-primitives)
    // are not updated, then 
    // a new instacne is created with the updated values
    // the following code is not allowed

    // println!("{}", my_user.email);

    // we can however reference the fields that were not transferred to second_user

    println!("{}", my_user.name);
    
    let new_point = Point(10, 12);
    println!("{}", new_point.1);

    let _new_unitlike = UnitLike;
}

// this derive statement applies to the User struct
// this allows for the use of Debug formatting with the println! macro
#[derive(Debug)]
struct User {
    name: String, 
    email: String,
    is_active: bool,
    login_count: i64
}

impl User {
    fn describe(&self) -> String {
        let mut my_str = String::new();
        my_str.push_str(&format!("Hi, my name is {}, my email is {}, I have logged on {} times, and my status is {}", &self.name, &self.email, &self.login_count, &self.is_active));
        my_str
    }
}

// this is a Tuple Struct
// no field names, just data types
// used to create a kind of Type
struct Point(i32, i32);

// this is a Unit-Like Struct
// behaves like a () unit
struct UnitLike;

 println! uses a reference to data, not the data itself
 this means that references can be passed to println! then used later
 dbg! prints out data but is passed ownership unless otherwise specified as &(reference)
 update syntax allows us to avoid unnecessary rewriting of struct fields we do not want to
 update
 similar to spread syntax in js
 this println! macro uses the :? to tell println to display the second_user
 using the debug default formatting
 this uses a debug format that displays each field on its own line
 note that because we did not change the email field,
 my_user is no longer valid as a whole
 this is because we transferred ownership of the email field to second_user
 when using update syntax, references used in the previous instance of the struct will be destroyed if any
 fields that do not implement the Copy() trait (i.e. non-primitives)
 are not updated, then 
 a new instacne is created with the updated values
 the following code is not allowed
 println!("{}", my_user.email);
 we can however reference the fields that were not transferred to second_user
 this derive statement applies to the User struct
 this allows for the use of Debug formatting with the println! macro
 this is a Tuple Struct
 no field names, just data types
 used to create a kind of Type
 this is a Unit-Like Struct
 behaves like a () unit

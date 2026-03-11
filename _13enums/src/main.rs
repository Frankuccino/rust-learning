// Enums
// an enums is a versatile tool used to represent a type that can take one of several possible variants.
// They offer basically a structured way to handle different kinds of data under a unified type which can be particularly useful for managing related values with distinct characteristics.
// You might be thinking that this looks like structs we have studied from the last lesson. In fact we can compare this to structs which are used to bundle related data fields, or if you remember we said that they're used to name and group related data fields but and enum..

// an enum does more than that, it extends this by allowing each variant to hold different types and amounts of data.

// using enum this way, is very appropriate. Why? Because an IP address can only be one version at a time. Despite both being IP addresses, either V4, or V6.

// So this situation illustrates how enums in Rust allow a value to be one of several predefined variants.

fn main() {
    enum IpAddressVersion {
        V4,
        V6 
    }

    let _four = IpAddressVersion::V4;
    let _six = IpAddressVersion::V6;


    // This IpAddressVersion defines that IP Address could be V4 or V6.
    // Create an instance of each variant of this Ip Address

    // You can use enums also in functions, so you can use these enums as parameters in any function actually.

    fn route(_ip_version: IpAddressVersion) {

    }

    route(IpAddressVersion::V4);
    route(IpAddressVersion::V6);

    // Think more about this IP address type and at the moment we didn't have a way to store the actual IP address data.
    // You might want to think of using structs to tackle the problem, we have learned everything about structs from the last lesson. If we will use structs for example.

    // Using structs - to solve that problem...
    struct IpAddress{
        version: IpAddressVersion,
        address: String

    }
    // We actually given a type to that version variant which is an enum. We passed the IpAddressVersion as a type for the version variant inside the IpAddress Struct.

    let home = IpAddress {
        version: IpAddressVersion::V4,
        address: String::from("127.0.0.1")
    }
    
    let loopback = IpAddress {
        version: IpAddressVersion::V6,
        address: String::from("::1")
    }

    // That's one way to use struct to solve the problem for storing data. We have stored that local address data and also the type of IP Address v4 or v6. But actually, Rust allows you to store data directly in each enum variant. You don't have to use structs.


    // The way to do that, inside the enum itself, we can actually define that datatype of V4, and V6.
    enum IpAddressVersion0 {
        V4(String),
        V6(String)
    }
    let home0 = IpAddressVersion0::V4(String::from("127.0.0.1"));
    let loopback0 = IpAddressVersion0::V6(String::from("::1"));
    // By using enums, each variant here of the Ip address can hold a string, and this string represents the IP address.
    // This is more concise and clean, instead of doing all that from the above (structs.)

    // Enhanced Enums Example
    enum IpAddressVersion1 {
        V4(u8,u8,u8,u8),
        V6(String)
    }
    let home1 = IpAddressVersion1::V4(127,0,0,1);
    let loopback1 = IpAddressVersion1::V6(String::from("::1"));
    

}

// So far we have studied the following:

/* Fundamentals
Primitive Data Types
Compound Data Types
Functions
Ownership
Borrowing and References
Variables
Constants
Shadowing
Comments
If Expression [control flow] / Break / continue
Loop / While Loop / For Loop
Structs
*/
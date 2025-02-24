// structs1.rs
// Address all the TODOs to make the tests pass!

struct ColorClassicStruct(u64, u64);
// TODO: Something goes here

struct ColorTupleStruct(String, String /* TODO: Something goes here */);

#[derive(Debug)]
struct UnitStruct;

struct Green {
    name: String,
    hex: String,
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // TODO: Instantiate a classic c struct!
        let green = Green {
            name: String::from("green"),
            hex: String::from("#00FF00"),
        };

        assert_eq!(green.name, "green");
        assert_eq!(green.hex, "#00FF00");
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct!

        let green = ColorTupleStruct(String::from("green"), String::from("#00FF00"));

        assert_eq!(green.0, "green");
        assert_eq!(green.1, "#00FF00");
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit struct!
        let unit_struct = UnitStruct;
        let message = format!("{:?}s are fun!", unit_struct);

        assert_eq!(message, "UnitStructs are fun!");
    }
}

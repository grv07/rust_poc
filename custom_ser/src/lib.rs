use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct CustomSerializer<'a> {
    name: String,
    id: &'a str,
    pass: &'a str
}

impl<'a> Serialize for CustomSerializer<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> 
    where 
        S: Serializer {
        let mut cs = serializer.serialize_struct("CustomSerializer", 3)?;
        let pass = format!("this-{}", &self.pass);
        cs.serialize_field("name", &self.name)?;
        cs.serialize_field("id", &self.id)?;
        cs.serialize_field("pass", &pass)?;
        cs.end()
    }
}

impl<'a> CustomSerializer<'a> {

   fn new(name:String, id: &'a str, pass:&'a str) -> Self {
        Self {
            name: name,
            id: id,
            pass: pass
        }
   }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let name = String::from("Custom");
        let id = "myid";
        let pass = "mypass";

        let cs = CustomSerializer::new(name, id, pass);
        let cs_s = serde_json::to_string(&cs);
        println!("{:?}", cs_s);
    }
}

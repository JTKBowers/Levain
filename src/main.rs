mod category;
use crate::category::Category;

struct Example;

impl Category for Example {
    type Error = ();
    fn name(&self) -> Result<String, ()> {
        return Ok("Example".to_string());
    }
    fn get_entries(&self) -> Result<Vec<String>, ()> {
        return Ok(
            vec![
                "An Entry".to_string(),
                "Another Entry".to_string(),
                "A Third Entry".to_string()
            ]
        )
    }
    fn launch(&self, entry: &String) -> Result<(), ()> {
        println!("Launching \"{}\"", entry);
        Ok(())
    }
}

fn main() {
    let category = Example{};

    for c in category.get_entries().unwrap().into_iter() {
        println!("{:?}", c);
    }
}

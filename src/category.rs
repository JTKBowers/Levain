type Entry = String;

pub trait Category {
    type Error;

    fn name(&self) -> Result<String, Self::Error>;
    fn get_entries(&self) -> Result<Vec<Entry>, Self::Error>;
    fn launch(&self, entry: &Entry) -> Result<(), Self::Error>;
}
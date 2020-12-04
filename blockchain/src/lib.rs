use ring::signature::Signature;
use ring::digest::Digest;

pub type DynBlock = Box<dyn Block>;

pub trait Block {
    fn hash(&self) -> Digest;
}

pub trait Blockchain {
    fn insert(&mut self, block: DynBlock);
    fn get(&self, key: usize) -> Option<DynBlock>;
    fn save(&self, filepath: String) -> std::result::Result<(), String>;
    fn load(&self, filepath: String) -> std::result::Result<(), String>;
}

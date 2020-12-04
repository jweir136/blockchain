use ring::signature::Signature;
use ring::digest::Digest;

pub trait Block {
    fn hash(&self) -> Digest;
    fn signature(&self) -> Option<Signature>;
}

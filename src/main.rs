use ckb_merkle_mountain_range;
use ckb_merkle_mountain_range::{Error, Merge, MMR, Result};
use ckb_merkle_mountain_range::util::MemStore;
use bytes::Bytes;

use blake2b_rs::{Blake2b, Blake2bBuilder};

fn new_blake2b() -> Blake2b {
    Blake2bBuilder::new(32).build()
}

#[derive(Eq, PartialEq, Clone, Debug, Default)]
struct Version {
    hash: Bytes,
    version: u32
}

#[derive(Eq, PartialEq, Clone, Debug, Default)]
struct NumberHash(pub Vec<Version>);
impl TryFrom<u32> for NumberHash {
    type Error = Error;
    fn try_from(num: u32) -> Result<Self> {
        let mut hasher = new_blake2b();
        let mut hash = [0u8; 32];
        hasher.update(&num.to_le_bytes());
        hasher.finalize(&mut hash);
        Ok(NumberHash(vec![Version{
            version: 0,
            hash: hash.to_vec().into()
        }]))
    }
}

struct MergeNumberHash;

impl Merge for MergeNumberHash {
    type Item = NumberHash;
    fn merge(lhs: &Self::Item, rhs: &Self::Item) -> Result<Self::Item> {
        let mut hasher = new_blake2b();
        let mut hash = [0u8; 32];
        hasher.update(&lhs.0.last().unwrap().hash);
        hasher.update(&rhs.0.last().unwrap().hash);
        hasher.finalize(&mut hash);
        Ok(NumberHash(vec![Version{
            version: 0,
            hash: hash.to_vec().into()
        }]))
    }
}

fn main() {
    let store = MemStore::default();
    let mut mmr = MMR::<_, MergeNumberHash, _>::new(0, &store);

    let ids: Vec<_> = (0u32..50u32).map(|i| {
        let awd = NumberHash::try_from(i).unwrap();

        println!("test: {awd:?}");

        mmr.push(awd).unwrap()
    }).collect();

    println!("{:?}", ids);

    let test = mmr.get_root().unwrap();
    println!("{:?}", test)

}

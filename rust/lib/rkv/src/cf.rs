//use paste::paste;

#[derive(Debug)]
pub struct ColumnFamily(pub *mut librocksdb_sys::rocksdb_column_family_handle_t);

impl rocksdb::AsColumnFamilyRef for ColumnFamily {
  fn inner(&self) -> *mut librocksdb_sys::rocksdb_column_family_handle_t {
    self.0
  }
}

unsafe impl Send for ColumnFamily {}
unsafe impl Sync for ColumnFamily {}

pub trait Cf<const N: usize> {
  fn new(db: &rocksdb::OptimisticTransactionDB) -> Self;
  fn li() -> [String; N];
}

#[macro_export]
macro_rules! column_family {

  ($($name:ident),*) => {
    #[derive(Debug)]
    pub struct Cf {
      $( pub $name:rkv::ColumnFamily ),*
    }

    const CF_N:usize = util::count!($($name),+);
    impl rkv::Cf<CF_N> for Cf {
      fn li() -> [String;CF_N] {
        [$(stringify!($name).into()),*]
      }
      fn new(db:&rocksdb::OptimisticTransactionDB) -> Cf {
        use rocksdb::AsColumnFamilyRef;
        Cf {
          $(
            $name:rkv::ColumnFamily(db.cf_handle(stringify!($name)).unwrap().inner())
          ),*
        }
      }
    }
  }

}

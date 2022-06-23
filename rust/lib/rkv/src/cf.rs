//use paste::paste;

#[macro_export]
macro_rules! count {
  ($name:ident) => { 1 };
  ($first:ident, $($rest:ident),*) => {
    1 + count!($($rest),*)
  }
}

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

    use rkv::ColumnFamily;

    pub struct Cf {
      $( pub $name:ColumnFamily ),*
    }

    impl rkv::Cf for Cf<count!($($name),+)> {
      fn li() -> Iter {
        [$(stringify!($name)),*].into_iter()
      }
      fn new(db:&rocksdb::OptimisticTransactionDB) -> Cf {
        use rocksdb::AsColumnFamilyRef;
        Cf {
          $(
            $name:ColumnFamily(db.cf_handle($name).unwrap().inner())
          ),*
        }
      }
    }
  }

}

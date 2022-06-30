use crate::cf;
use anyhow::Result;
use rocksdb::{
  BlockBasedOptions, Cache, DBCompactionStyle, DBCompressionType, OptimisticTransactionDB, Options,
  SingleThreaded, Transaction, DB,
};
use std::{collections::BTreeSet, path::PathBuf};

#[derive(Debug)]
pub struct Kv<Cf: cf::Cf<N>, const N: usize> {
  pub db: OptimisticTransactionDB,
  pub cf: Cf,
}

impl<Cf: cf::Cf<N>, const N: usize> util::kv::Kv for Kv<Cf, N> {
  fn get(&self, key: &[u8]) -> Option<Box<[u8]>> {
    if let Ok(Some(val)) = err::ok!(self.db.get(key)) {
      return Some(val.into());
    }
    None
  }

  fn set(&self, key: &[u8], val: &[u8]) {
    err::log!(self.db.put(key, val));
  }
}

impl<Cf: cf::Cf<N>, const N: usize> Kv<Cf, N> {
  #[allow(invalid_value)]
  pub fn new(path: impl Into<PathBuf>) -> Self {
    let mut db = Kv {
      db: open(path, Cf::li()).unwrap(),
      cf: unsafe { std::mem::MaybeUninit::uninit().assume_init() },
    };
    let ptr: *const OptimisticTransactionDB = &db.db;
    db.cf = Cf::new(unsafe { &*ptr });
    db
  }
  pub fn tx(&self) -> Transaction<OptimisticTransactionDB> {
    self.db.transaction()
  }
  pub fn with_tx<T>(
    &self,
    run: impl Fn(&Transaction<OptimisticTransactionDB>) -> Result<T>,
  ) -> Result<T> {
    let tx = self.tx();
    match err::ok!(run(&tx)) {
      Ok(r) => {
        tx.commit()?;
        return Ok(r);
      }
      Err(err) => {
        err::log!(tx.rollback());
        return Err(err);
      }
    }
  }
}

fn open(
  path: impl Into<PathBuf>,
  cf_li: impl IntoIterator<Item = String>,
) -> Result<OptimisticTransactionDB> {
  let cpu = num_cpus::get() as _;
  let mut opt = Options::default();

  opt.create_if_missing(true);
  opt.set_use_fsync(false);
  opt.set_compaction_style(DBCompactionStyle::Universal);
  opt.set_disable_auto_compactions(false);
  opt.increase_parallelism(cpu);
  opt.set_max_background_jobs(cpu / 3 + 1);
  opt.set_keep_log_file_num(16);
  opt.set_level_compaction_dynamic_level_bytes(true);

  opt.set_compression_type(DBCompressionType::Lz4);
  opt.set_bottommost_compression_type(DBCompressionType::Zstd);
  /*
  RocksDB documenation says that 16KB is a typical dictionary size.
  We've empirically tuned the dicionary size to twice of that 'typical' size.
  Having train data size x100 from dictionary size is a recommendation from RocksDB.
  See: https://rocksdb.org/blog/2021/05/31/dictionary-compression.html?utm_source=dbplatz

  We use default parameters of RocksDB here:
  window_bits is -14 and is unused (Zlib-specific parameter),
  compression_level is 32767 meaning the default compression level for ZSTD,
  compression_strategy is 0 and is unused (Zlib-specific parameter).
  See: https://github.com/facebook/rocksdb/blob/main/include/rocksdb/advanced_options.h#L176:
  */
  let dict_size = 32768;
  let max_train_bytes = dict_size * 128;
  opt.set_bottommost_compression_options(-14, 32767, 0, dict_size, true);
  opt.set_bottommost_zstd_max_train_bytes(max_train_bytes, true);

  opt.set_enable_blob_files(true);
  opt.set_min_blob_size(4096);
  opt.set_blob_file_size(268435456);
  opt.set_blob_compression_type(DBCompressionType::Zstd);
  opt.set_enable_blob_gc(true);
  opt.set_blob_gc_age_cutoff(0.25);
  opt.set_blob_gc_force_threshold(0.8);

  opt.set_bytes_per_sync(8388608);
  opt.optimize_for_point_lookup(1024 * 1024);

  let cache = Cache::new_lru_cache(16 * 1024 * 1024)?;
  let mut bopt = BlockBasedOptions::default();
  // https://rocksdb.org/blog/2021/12/29/ribbon-filter.html
  bopt.set_ribbon_filter(10.0);
  bopt.set_block_cache(&cache);
  bopt.set_block_size(6 * 1024);
  bopt.set_cache_index_and_filter_blocks(true);
  bopt.set_pin_l0_filter_and_index_blocks_in_cache(true);
  opt.set_block_based_table_factory(&bopt);
  opt.create_missing_column_families(true);

  let path = path.into();

  // https://blog.petitviolet.net/post/2021-03-25/use-rocksdb-from-rust
  let li = DB::list_cf(&opt, &path).unwrap_or_default();
  let mut db: OptimisticTransactionDB<SingleThreaded> =
    OptimisticTransactionDB::open_cf(&opt, &path, &li)?;
  let cf_set = BTreeSet::from_iter(li);

  for i in cf_li {
    if cf_set.get(&i).is_none() {
      db.create_cf(i, &opt)?;
    }
  }

  Ok(db)
}

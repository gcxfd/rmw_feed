use rkv::column_family;

column_family!(
  // 自增主键
  id,
  // 用户
  user_pk_id,
  user_id_sk,
  user_id_name,
  // 房间
  room_pk_id,
  room_id_sk,
  room_id_name
);

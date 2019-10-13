CREATE TABLE objects(
  id SERIAL PRIMARY KEY,
  position_x real DEFAULT 0,
  position_y real DEFAULT 0,
  position_z real DEFAULT 0,
  rotation_x real DEFAULT 0,
  rotation_y real DEFAULT 0,
  rotation_z real DEFAULT 0,
  scale_x real DEFAULT 1,
  scale_y real DEFAULT 1,
  scale_z real DEFAULT 1,
  mass real DEFAULT 1,
  velocity_x real DEFAULT 0,
  velocity_y real DEFAULT 0,
  velocity_z real DEFAULT 0,
  collision_x real DEFAULT 0,
  collision_y real DEFAULT 0,
  collision_z real DEFAULT 0,
  height real DEFAULT 0,
  radius real DEFAULT 0
)
-- Database optimization (less memory):
-- Optional fields at the end
-- variable types before optionals
CREATE TABLE objects(
  id SERIAL PRIMARY KEY,
  x int NOT NULL,
  y int NOT NULL,
  z int NOT NULL,
  scale_x real,
  scale_y real,
  scale_z real,
  height real,
  radius real
)
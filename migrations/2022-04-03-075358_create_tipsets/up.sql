CREATE TABLE tipsets (
  id SERIAL PRIMARY KEY,
  height BIGINT,
  cids JSONB[],
  blocks JSONB[]
);
CREATE INDEX height_index ON tipsets (height);

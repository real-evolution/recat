CREATE TABLE products (
  id                UUID        NOT NULL PRIMARY KEY,
  title             VARCHAR     NOT NULL,
  quantity          INTEGER     NOT NULL,
  description       VARCHAR     NULL,
  owner_id          UUID        NOT NULL,
  price_token_id    UUID        NOT NULL,
  price_amount      DECIMAL     NOT NULL,
  created_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at        TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX ON products USING btree (created_at);

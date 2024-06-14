-- Your SQL goes here
CREATE TABLE peers
(
    id        uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    name     VARCHAR NOT NULL,
    enabled BOOLEAN NOT NULL DEFAULT TRUE ,
    persistent_keepalive    INTEGER NOT NULL DEFAULT 25,
    allowed_ips      VARCHAR NOT NULL DEFAULT '0.0.0.0/0, ::/0',
    preshared_key   VARCHAR ,
    private_key VARCHAR NOT NULL ,
    public_key  VARCHAR NOT NULL ,
    if_pubkey  VARCHAR NOT NULL ,
    address VARCHAR NOT NULL ,
    transfer_rx INTEGER NOT NULL DEFAULT 0,
    transfer_tx INTEGER NOT NULL DEFAULT 0,
    last_handshake_at TIMESTAMP ,
    endpoint_addr   VARCHAR,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)


--    interface_id = mapped_column(ForeignKey("interface.id", ondelete = "CASCADE"))
--    interface: Mapped["Interface"] = relationship(back_populates = "peers")
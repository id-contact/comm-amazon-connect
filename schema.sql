DROP TABLE IF EXISTS "session";

CREATE TABLE "session" (
    "id" SERIAL NOT NULL,
    "dtmfcode" text NOT NULL,
    "resultcode" text NOT NULL,
    "sessionid" text,
    "purpose" text,
    "attr_jwt" text,
    PRIMARY KEY ("id")
);

CREATE UNIQUE INDEX ON "session" ("dtmfcode");
CREATE UNIQUE INDEX ON "session" ("sessionid");

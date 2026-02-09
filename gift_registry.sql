BEGIN TRANSACTION;
DROP TABLE IF EXISTS "items";
CREATE TABLE IF NOT EXISTS "items" (
	"id"	TEXT,
	"name"	TEXT NOT NULL DEFAULT "",
	"quantity"	INTEGER NOT NULL DEFAULT 1,
	"priority"	TEXT CHECK("priority" IN ("low", "medium", "high", "highest")) NOT NULL DEFAULT "medium",
	"url"	TEXT NULL DEFAULT NULL,
	PRIMARY KEY("id")
) STRICT;
DROP TABLE IF EXISTS "notes";
CREATE TABLE IF NOT EXISTS "notes" (
	"id"	INTEGER,
	"note"	TEXT,
	PRIMARY KEY("id" AUTOINCREMENT)
) STRICT;
DROP TABLE IF EXISTS "item_notes";
CREATE TABLE IF NOT EXISTS "item_notes" (
	"item_id"	TEXT,
	"note_id"	INTEGER,
	PRIMARY KEY("item_id","note_id")
) STRICT;
INSERT INTO "priorities" VALUES (1,'low'),
 (2,'medium'),
 (3,'high'),
 (4,'highest');
COMMIT;

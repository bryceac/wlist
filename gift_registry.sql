BEGIN TRANSACTION;
DROP TABLE IF EXISTS "items";
CREATE TABLE IF NOT EXISTS "items" (
	"id"	TEXT,
	"name"	TEXT,
	"quantity"	INTEGER,
	"priority"	INTEGER,
	"url"	TEXT,
	PRIMARY KEY("id")
);
DROP TABLE IF EXISTS "priorities";
CREATE TABLE IF NOT EXISTS "priorities" (
	"id"	INTEGER,
	"priority"	TEXT,
	PRIMARY KEY("id" AUTOINCREMENT)
);
DROP TABLE IF EXISTS "notes";
CREATE TABLE IF NOT EXISTS "notes" (
	"id"	INTEGER,
	"note"	TEXT,
	PRIMARY KEY("id" AUTOINCREMENT)
);
DROP TABLE IF EXISTS "item_notes";
CREATE TABLE IF NOT EXISTS "item_notes" (
	"item_id"	TEXT,
	"note_id"	INTEGER,
	PRIMARY KEY("item_id","note_id")
);
INSERT INTO "priorities" VALUES (1,'low'),
 (2,'medium'),
 (3,'high'),
 (4,'highest');
DROP VIEW IF EXISTS "registry";
CREATE VIEW registry AS SELECT i.id, i.name, i.quantity, p.priority, i.url FROM items as i JOIN priorities as p WHERE i.priority = p.id ORDER BY p.id DESC;
COMMIT;

-- Your SQL goes here
CREATE TABLE IF NOT EXISTS to_do_items (
    "id" uuid NOT NULL,
    "title" varchar(255),
    "note" varchar(255),
    CONSTRAINT "PK_ToDoItems" PRIMARY KEY ("id")
);

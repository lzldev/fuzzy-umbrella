import { relations, sql } from "drizzle-orm";
import { pgTable, timestamp, uuid, varchar } from "drizzle-orm/pg-core";

import { sqliteTable, text } from "drizzle-orm/sqlite-core";
export const users2 = sqliteTable("users", {
  id: text("ID"),
});

export const users = pgTable("users", {
  id: uuid("id").primaryKey().unique().defaultRandom(),
  email: varchar("email").unique().notNull(),
  username: varchar("username").notNull(),
  image_url: varchar("image_url"),
  clerk_id: varchar("clerk_id").unique().notNull(),
  clerk_updated_at: timestamp("clerk_updated_at").defaultNow(),
});

export const usersRelations = relations(users, ({ many }) => ({
  posts: many(posts),
}));

export const posts = pgTable("posts", {
  id: uuid("id").primaryKey().unique().defaultRandom(),
  content: varchar("content").notNull(),
  imageKey: varchar("image_key").notNull(),
  createdAt: varchar("created_at")
    .default(sql`CURRENT_TIMESTAMP`)
    .notNull(),
  userId: uuid("user_id")
    .notNull()
    .references(() => users.id, { onDelete: "cascade" }),
});

export const postsRelations = relations(posts, ({ many, one }) => ({
  author: one(users, {
    fields: [posts.userId],
    references: [users.id],
  }),
}));

export type InsertUser = typeof users.$inferInsert;
export type SelectUser = typeof users.$inferSelect;

export type InsertPost = typeof posts.$inferInsert;
export type SelectPost = typeof posts.$inferSelect;

export type Schema = {
  posts: typeof posts;
  users: typeof users;
};

export type SomethingElse = {
  ["isTable"]: true;
};

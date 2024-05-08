import { relations, sql } from "drizzle-orm";
import { integer, sqliteTable, text } from "drizzle-orm/sqlite-core";

export const users = sqliteTable("users", {
  id: integer("id").primaryKey(),
  email: text("email").unique().notNull(),
  username: text("username").notNull(),
  image_url: text("image_url"),
  clerk_id: text("clerk_id").unique().notNull(),
  clerk_updated_at: integer("clerk_updated_at")
    .notNull()
    .default(sql`CURRENT_TIMESTAMP`),
});

export const usersRelations = relations(users, ({ many }) => ({
  posts: many(posts),
}));

export const posts = sqliteTable("posts", {
  id: text("id", {
    mode: "text",
  })
    .unique()
    .primaryKey(),
  content: text("content", {
    length: 150,
  }).notNull(),
  imageKey: text("image_key").notNull(),
  createdAt: text("created_at")
    .default(sql`CURRENT_TIMESTAMP`)
    .notNull(),
  userId: integer("user_id")
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

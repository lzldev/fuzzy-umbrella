ALTER TABLE `users` RENAME COLUMN `name` TO `clerk_username`;--> statement-breakpoint
ALTER TABLE users ADD `image_url` text;--> statement-breakpoint
ALTER TABLE users ADD `clerk_id` text NOT NULL;--> statement-breakpoint
ALTER TABLE users ADD `clerk_updated_at` integer DEFAULT CURRENT_TIMESTAMP NOT NULL;--> statement-breakpoint
CREATE UNIQUE INDEX `users_clerk_id_unique` ON `users` (`clerk_id`);
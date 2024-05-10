export function createPostKey(postId: string) {
  return `post:create:${postId}`;
}

export function clerkIdtoUserIdKey(userId: string) {
  return `user:clerk_id:${userId}`;
}

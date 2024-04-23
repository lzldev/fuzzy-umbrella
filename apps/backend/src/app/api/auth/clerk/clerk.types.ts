export type ClerkJWTPayload = {
  /**
   * Clerk Session ID
   */
  sid: string;

  /**
   * Clerk User ID
   */
  sub: string;
};

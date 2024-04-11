import 'dotenv/config';
import { createConnection } from './connection';

export const db = createConnection();

export type DB = typeof db;

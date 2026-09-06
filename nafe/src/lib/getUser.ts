import type { RowDataPacket } from 'mysql2';
import { db, type User } from './db';

export default async function (userIdOrEmail: string | number): // Accept userIdOrEmail as string or number
Promise<User | null> { // Return promise of User or null
    const con = await db.getConnection(); // get con

    // select user query
    // select where either the id or email matches the arg
    const query = await con.prepare('SELECT * FROM users WHERE id = ? OR email = ?');
    const [rows] = await query.execute([userIdOrEmail, userIdOrEmail]) as RowDataPacket[]; // get data rows

    con.release() // release connection

    if (rows.length === 0) {
        return null; // return null if no user found found
    }

    return rows[0]; // return first row
}
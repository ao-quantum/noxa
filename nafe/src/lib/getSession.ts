import type { RowDataPacket } from 'mysql2';
import { db, type Session } from './db';

export default async function (id: string): Promise<Session | null> { // accept id of session, return session or null
    const con = await db.getConnection(); // get con

    // Get session query
    const query = await con.prepare('SELECT * FROM sessions WHERE id = ?');
    const [rows] = await query.execute([id]) as RowDataPacket[]; // get data rows

    con.release() // release connection

    if (rows.length === 0) {
        return null; // return null if no rows found
    }

    return rows[0]; // return first row
}
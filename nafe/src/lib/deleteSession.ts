import { db } from './db';

/**
 * Delete a session from the database
 * @param sessionId ID of the session to delete
 * @returns ID of the session that was deleted
 */
export default async function (sessionId: string) {
    const con = await db.getConnection(); // get db con

    // Run delete query
    const query = await con.prepare('DELETE FROM sessions WHERE id = ?');
    await query.execute([sessionId]);

    con.release() // release connection
    
    return sessionId;
}
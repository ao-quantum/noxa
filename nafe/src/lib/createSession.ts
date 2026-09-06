import { env } from '$env/dynamic/private'
import { db } from './db';

export default async function (ip: string, userId: number) { // accept ip and userId to create session
    const con = await db.getConnection(); // get connection

    // Generate a random session ID. Random string of 30 characters
    const sessionId = Math.random().toString(36).substring(2, 15) + Math.random().toString(36).substring(2, 15);

    // Create expire date
    const expire = new Date();
    
    if (env.DEBUG) {
        // Expire in 1 hour in debug mode
        expire.setHours(expire.getHours() + 1);
    } else {
        // Expire in 2 days in debug mode
        expire.setHours(expire.getHours() + 48);
    }

    // Create session query
    const query = await con.prepare('INSERT INTO sessions (id, userid, ip, expire) VALUES (?, ?, ?, ?)');
    await query.execute([sessionId, userId, ip, expire]);

    con.release() // release connection
    
    return {
        id: sessionId,
        expire
    };
}
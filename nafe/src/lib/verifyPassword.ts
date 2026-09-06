import bcrypt from 'bcryptjs'
import { db } from './db';
import type { RowDataPacket } from 'mysql2';

export default async function (idOrEmail: string | number, inputPass: string): // Accept idOrEmail as string or number and inputPass as string (password to check)
Promise<boolean> { // Return promise of boolean, if password matches
    const con = await db.getConnection(); // get con

    // Fetch the password hash from the database
    const query = await con.prepare('SELECT password FROM users WHERE id = ? OR email = ?');
    const [rows] = await query.execute([idOrEmail, idOrEmail]) as RowDataPacket[];

    con.release() // release connection

    if (rows.length === 0) {
        return false; // if no user found then invalid
    }

    const dbPassHash = rows[0].password; // choose password from first row

    return bcrypt.compareSync(inputPass, dbPassHash); // compare hash and inputPass, return output
}

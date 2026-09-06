import * as mysql from 'mysql2';
import {
    MYSQL_USER,
    MYSQL_PASSWORD,
    MYSQL_DATABASE,
    MYSQL_HOST,
    MYSQL_PORT
} from '$env/static/private';

const connUri = `mysql://${MYSQL_USER}:${MYSQL_PASSWORD}@${MYSQL_HOST}:${MYSQL_PORT}/${MYSQL_DATABASE}`; // Construct con URI

const client = mysql.createPool(connUri); // create mysql connection pool
const pool = client.promise(); // make pool available as promises

export const db = pool;

// Card table interface
export interface User {
    id: number;
    email: string;
    username: string;
    password: string;
    createdat: Date;
    updatedat: Date;
}

// Session table interface
export interface Session {
    id: string;
    userid: number;
    ip: string;
    expire: Date;
    createdat: Date;
}

export async function migrate() {
    const con = await pool.getConnection(); // get con from pool

    // Create users table
    const createUsersQ = await con.prepare(`
        CREATE TABLE IF NOT EXISTS \`users\` (
            \`id\` int(11) NOT NULL AUTO_INCREMENT,
            \`email\` varchar(255) NOT NULL UNIQUE,
            \`username\` varchar(255) NOT NULL,
            \`password\` varchar(255) NOT NULL,
            \`createdat\` datetime DEFAULT current_timestamp(),
            \`updatedat\` datetime DEFAULT current_timestamp() ON UPDATE current_timestamp(),
            PRIMARY KEY (\`id\`)
            );
        `)

    // Create sessions table
    const createSessionsQ = await con.prepare(`
        CREATE TABLE IF NOT EXISTS \`sessions\` (
            \`id\` varchar(255) NOT NULL,
            \`userid\` int(11) NOT NULL,
            \`ip\` varchar(255) NOT NULL,
            \`expire\` datetime NOT NULL,
            \`createdat\` datetime DEFAULT current_timestamp(),
            
            PRIMARY KEY (\`id\`),
            FOREIGN KEY (\`userid\`)
                REFERENCES \`users\`(\`id\`)
                ON UPDATE CASCADE ON DELETE CASCADE
            );
        `);

    // Create sessions table
    const createCardTableQ = await con.prepare(`
        CREATE TABLE IF NOT EXISTS \`cards\` (
            \`id\` int(11) NOT NULL AUTO_INCREMENT,
            \`userid\` int(11) NOT NULL,
            \`front\` varchar(255) NOT NULL,
            \`back\` text NOT NULL,
            \`s\` FLOAT,
            \`d\` FLOAT,
            \`r\` FLOAT,
            \`last_review\` DATETIME,
            \`createdat\` datetime DEFAULT current_timestamp(),
            \`updatedat\` datetime DEFAULT current_timestamp() ON UPDATE current_timestamp(),
            PRIMARY KEY (\`id\`),
            FOREIGN KEY (\`userid\`)
                REFERENCES \`users\`(\`id\`)
                ON UPDATE CASCADE ON DELETE CASCADE
        );
    `);

    // Create sessions table
    const createCardRecallsTableQ = await con.prepare(`
        CREATE TABLE IF NOT EXISTS \`cardrecalls\` (
            \`id\` int(11) NOT NULL AUTO_INCREMENT,
            \`cardid\` int(11) NOT NULL,
            \`grade\` INT NOT NULL,
            \`createdat\` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP(),
            PRIMARY KEY (\`id\`),
            FOREIGN KEY (\`cardid\`)
                REFERENCES \`cards\`(\`id\`)
                ON UPDATE CASCADE ON DELETE CASCADE
        );
    `);

    try {
        // Run all queries
        createUsersQ.execute([]);
        createSessionsQ.execute([]);
        createCardTableQ.execute([]);
        createCardRecallsTableQ.execute([]);
    } catch (e) {
        console.error(e);
        throw e;
    }

    con.release()
}

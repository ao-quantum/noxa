import getSession from '$lib/getSession';
import getUser from '$lib/getUser';
import { redirect, type Handle } from '@sveltejs/kit';

// Routes accessible only to public users
// (not logged in users)
const PUBLIC_ROUTES = [
    '/',
    '/login',
    '/migrate',
]

// This hook runs on every request
export const handle: Handle = async ({ event, resolve }) => {
    const sessionId = event.cookies.get('session'); // get session cookie

    if (sessionId) { // if there is a session cookie (probably logged in user)
        const session = await getSession(sessionId); // get session from db
    
        if (!session) { // if invalid session
            event.cookies.delete('session', { // delete session cookie
                path: '/',
            });
            return redirect(302, '/'); // send back to login page
        }

        if (PUBLIC_ROUTES.includes(event.url.pathname)) { // if user is on a public only route
            return redirect(302, '/app'); // send back to /app
        }
    
        if (session.expire.getTime() < Date.now()) { // if session is expired
            event.cookies.delete('session', { // delete session cookie
                path: '/',
            });
            return redirect(302, '/'); // send back to login page
        }
    
        const user = await getUser(session.userid); // get user from db
    
        if (!user) { // if invalid user
            event.cookies.delete('session', { // delete session cookie
                path: '/',
            });
            return redirect(302, '/'); // send back to login page
        }
    
        // set user and sessionId in locals
        event.locals.user = user;
        event.locals.sessionId = sessionId;
    
        if (event.locals.user && PUBLIC_ROUTES.includes(event.url.pathname)) { // if user is logged in and on a public only route
            // this check is probably redundant but just in case
            return redirect(302, '/app'); // send back to /app
        }
    } else { // no session cookie (probably not logged in user)
        if (!PUBLIC_ROUTES.includes(event.url.pathname)) { // if user is not on a public only route
            return redirect(302, '/'); // send back to login page
        }
    }
    
    return resolve(event); // resolve the event
};

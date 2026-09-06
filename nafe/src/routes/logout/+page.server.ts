import deleteSession from '$lib/deleteSession';
import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ locals, cookies }) => {
    // if there is a logged in user with a valid session visiting this endpoint
    if (locals.user) {
        const sessionId = cookies.get('session'); // get session cookie
        if (sessionId) {
            deleteSession(sessionId); // delete session from db
            cookies.delete('session', { // delete session cookie
                path: '/',
            })
        }
    }

    return redirect(303, '/'); // send agent to login page
}

import createSession from '$lib/createSession';
import getUser from '$lib/getUser';
import verifyPassword from '$lib/verifyPassword';
import { type RequestHandler } from '@sveltejs/kit';

export const POST: RequestHandler = async ({ cookies, request, locals, getClientAddress }) => {
    // check that user is not already logged in
    if (locals.user) {
        return new Response(JSON.stringify({}), { // redirect to /app if logged in
            status: 303,
            headers: {
                'Content-Type': 'application/json',
                Location: '/app',
            },
        })
    }

    // copy data from form
    const data = await request.formData();
    const email = data.get('email');
    const password = data.get('password');

    // input validation
    if (!email || !password) {
        return new Response(JSON.stringify({
            success: false,
            message: 'Invalid email or password',
        }), {
            status: 400,
            headers: {
                'Content-Type': 'application/json',
            },
        });
    }

    // Fetch the user from the database with the said email from the request body
    const user = await getUser(email.toString()); // get user from db

    // If no user found from the database, send back failure JSON
    if (!user) {
        return new Response(JSON.stringify({ // invalid user
            success: false,
            message: 'Invalid email or password',
        }), {
            status: 400,
            headers: {
                'Content-Type': 'application/json',
            },
        });
    }

    // check password of user
    if (!(await verifyPassword(user.id, password.toString()))) {
        return new Response(JSON.stringify({ // if password is wrong
            success: false,
            message: 'Invalid email or password',
        }), {
            status: 400,
            headers: {
                'Content-Type': 'application/json',
            },
        });
    }

    const session = await createSession(getClientAddress(), user.id); // create session with user's ip and user id

    cookies.set('session', session.id, { // set cookie
        expires: session.expire,
        path: '/',
    });

    return new Response(JSON.stringify({ // redirect to /app after successful login
        success: true,
        redirect: '/app',
    }), {
        headers: {
            'Content-Type': 'application/json',
        },
    })
}
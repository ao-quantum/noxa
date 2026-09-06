import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = ({ locals }) => {
    return {
        username: locals.user?.username,
        sessionId: locals.sessionId,
    }
}
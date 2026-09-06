import { migrate } from '$lib/db';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async () => {
    try {
        await migrate();
        return {
            success: true
        };
    } catch (e) {
        console.error(e);
        return {
            success: false
        };
    }
    
};

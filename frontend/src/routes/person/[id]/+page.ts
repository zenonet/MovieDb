import { PUBLIC_API_URL } from '$env/static/public';

export const load = async ({ params }) => {

    const resp = await fetch(`${PUBLIC_API_URL}/person/${params.id}`);

    if(!resp.ok){
        return {
            success: false,
            error: await resp.text(),
            details: null,
        }
    }

    const res = await resp.json();

    return {
        success: true,
        error: null,
        details: res,
    };
};
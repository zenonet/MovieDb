import { getPersonsByName } from "$lib/services/movieService";

export const load = async () => {

    const persons =  (await getPersonsByName("")).data;
    return {
        persons
    }
};
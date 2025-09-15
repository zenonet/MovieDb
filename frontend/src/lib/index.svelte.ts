// place files you want to import through the `$lib` alias in this folder.

function createUserState() {
    let user = $state<User | null>(null);
    return {
        get user() {
            if (user === null) {
                const dat = localStorage.getItem("user");
                if (dat) user = JSON.parse(dat);
            }
            return user
        },
        set: (newUser: User) => {
            user = newUser
            localStorage.setItem("user", JSON.stringify(user));
        },
        reset: () => { 
            user = null;
            localStorage.removeItem("user");
        },
    }
}

export const userState = createUserState();


function createIdState() {
    let selectedId = $state<string | null>(null);
    return {
        get selectedId() {
            if (selectedId === null && typeof localStorage !== 'undefined') {
                const dat = localStorage.getItem("selectedId");
                selectedId = dat ?? "";
            }
            return selectedId
        },
        set: (id: string) => {
            selectedId = id
            if(typeof localStorage !== 'undefined') localStorage.setItem("selectedId", id);

        },
    }
}


export let selectedMovieInList = createIdState();
import type { UserInfo } from "./bindings/UserInfo";

class UserSession {
    user = $state<UserInfo | null>(null);
    loading = $state(true);

    async fetch() {
        try {
            const res = await fetch("/api/users/@me");
            if (!res.ok) throw new Error("Unable to fetch user info");
            this.user = await res.json();
        } catch (err) {
            console.log(err);
        } finally {
            this.loading = false;
        }
    }
}

export const session = new UserSession();

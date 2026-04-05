import { env } from "cloudflare:workers";
import type { Works } from "./model.js";
import type { D1Return } from "../commons/db-returns.js";

export class WorkService {
    private db;

    constructor() {
        this.db = env.palebride;
    }

    async getWorks() {
        const res: D1Return = await this.db
            .prepare("SELECT * FROM works WHERE deleted is NULL;")
            .run<Works>();

        return res.results;
    }
}

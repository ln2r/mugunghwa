import { env } from "cloudflare:workers";
import type { Works } from "./model.js";
import type { D1Return } from "../commons/db-returns.js";
import { Snowflake } from "@theinternetfolks/snowflake";

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

    async getWork(id: string) {
        const res: D1Return = await this.db
            .prepare("SELECT * FROM works WHERE id = ? AND deleted is NULL;")
            .bind(id)
            .run<Works>();

        return res.results;
    }

    async add(body: Works) {
        const id = Snowflake.generate();
        await this.db
            .prepare(
                "INSERT INTO works (id, title, description, url, stacks, created, updated) VALUES (?, ?, ?, ?, ?, ?, ?);",
            )
            .bind(
                id,
                body.title,
                body.description,
                body.url ?? null,
                body.stacks ?? null,
                new Date().toISOString(),
                new Date().toISOString(),
            )
            .run();

        const res: D1Return = await this.db
            .prepare("SELECT * FROM works WHERE id = ?;")
            .bind(id)
            .run<Works>();

        return res.results[0];
    }

    async update(id: string, body: Works) {
        const exist: D1Return = await this.db
            .prepare("SELECT id FROM works WHERE id = ? AND deleted is NULL;")
            .bind(id)
            .run<Works>();

        if (exist.results.length === 0) {
            return;
        }

        await this.db
            .prepare(
                "UPDATE works SET title = ?, description = ?, url = ?, stacks = ?, updated = ? WHERE id = ?;",
            )
            .bind(
                body.title,
                body.description,
                body.url,
                body.stacks,
                new Date().toISOString(),
                id,
            )
            .run();

        const res: D1Return = await this.db
            .prepare("SELECT * FROM works WHERE id = ?;")
            .bind(id)
            .run<Works>();

        return res.results[0];
    }
}

import { env } from "cloudflare:workers";
import type { D1Return } from "../commons/db-returns";
import type { Clipboards } from "./model";
import { Snowflake } from "@theinternetfolks/snowflake";

export class ClipboardService {
    private db;

    constructor() {
        this.db = env.palebride
    }

    async getClipboards() {
        const res: D1Return = await this.db
            .prepare("SELECT * FROM clipboards WHERE deleted is NULL;")
            .run<Clipboards>()

        return res.results;
    }

    async getClipboard(id: string) {
        const res: D1Return = await this.db
            .prepare("SELECT * FROM clipboards WHERE id = ? AND deleted is NULL;")
            .bind(id)
            .run<Clipboards>();

        return res.results;
    }

    async add(body: Clipboards) {
        const id = Snowflake.generate();
        const now = new Date();
        await this.db
            .prepare(
                "INSERT INTO clipboards (id, title, body, description, created, updated) VALUES (?, ?, ?, ?, ?, ?);",
            )
            .bind(id, body.title, body.body, body.description ?? null, now.toISOString(), now.toISOString()).run();

        const res: D1Return = await this.db
            .prepare("SELECT * FROM clipboards WHERE id = ?;")
            .bind(id)
            .run<Clipboards>();

            return res.results[0];
        }   

    async update(id: string, body: Clipboards) {
        const exists: D1Return = await this.db
            .prepare("SELECT id FROM clipboards WHERE id = ? AND deleted is NULL;")
            .bind(id)
            .run<Clipboards>();

        if (exists.results.length === 0) {
            return;
        }

        await this.db
            .prepare("UPDATE clipboards SET title = ?, body = ?, description = ?, updated = ? WHERE id = ?")
            .bind(
                body.title,
                body.body,
                body.description,
                new Date().toISOString(),
                id,
            )
            .run();

        const res: D1Return = await this.db
            .prepare("SELECT * FROM clipboards WHERE id = ?;")
            .bind(id)
            .run<Clipboards>();

        return res.results[0];
    }

    async delete(id: string) {
            const exist: D1Return = await this.db
                .prepare("SELECT id FROM clipboards WHERE id = ? AND deleted is NULL;")
                .bind(id)
                .run<Clipboards>();
    
            if (exist.results.length === 0) {
                return;
            }
    
            const res: D1Return = await this.db
                .prepare("UPDATE clipboards SET deleted = ? WHERE id = ?;")
                .bind(new Date().toISOString(), id)
                .run();
    
            return res.success;
        }
}
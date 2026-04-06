import Elysia from "elysia";
import { WorkService } from "./service.js";

const workService = new WorkService();

export const works = new Elysia({ prefix: "/works" })
    .get("/:id", async ({ status, params: { id } }) => {
        const work = await workService.getWork(id);
        if (work.length === 0) {
            throw status(404, "Work not found");
        }

        return work[0];
    })
    .get("/", () => workService.getWorks())
    .post("/", async ({ body }) => {
        return await workService.add(body);
    })
    .patch("/:id", async ({ status, body, params: { id } }) => {
        const res = await workService.update(id, body);

        if (!res) {
            throw status(404, "Work not found");
        }

        return res;
    });

import Elysia from "elysia";
import { WorkService } from "./service.js";
import { bearer } from "@elysiajs/bearer";
import { auth } from "../auth/index.js";

const workService = new WorkService();

export const works = new Elysia({ prefix: "/works" })
    .use(bearer())
    .use(auth)
    .get("/:id", async ({ status, params: { id } }) => {
        const work = await workService.getWork(id);
        if (work.length === 0) {
            throw status(404, "Work not found");
        }

        return work[0];
    })
    .get("/", () => workService.getWorks())
    .post(
        "/",
        async ({ body }) => {
            return workService.add(body);
        },
        {
            isSignedIn: true,
        },
    )
    .patch(
        "/:id",
        async ({ status, body, params: { id } }) => {
            const res = await workService.update(id, body);

            if (!res) {
                throw status(404, "Work not found");
            }

            return res;
        },
        {
            isSignedIn: true,
        },
    )
    .delete(
        "/:id",
        async ({ status, params: { id } }) => {
            const res = await workService.delete(id);

            if (!res) {
                throw status(404, "Work not found");
            }

            return "Deleted";
        },
        {
            isSignedIn: true,
        },
    );

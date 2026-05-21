import Elysia from "elysia";
import { bearer } from "@elysiajs/bearer";
import { WritingService } from "./service.js";
import { auth } from "../auth/index.js";

const writingService = new WritingService();

export const writings = new Elysia({ prefix: "/writings" })
    .use(bearer())
    .use(auth)
    .get("/:query", async ({ status, params: { query } }) => {
        const writing = await writingService.getWriting(query);

        if (!writing) {
            throw status(404, "Writing not found");
        }

        return writing;
    })
    .get("/", () => writingService.getWritings())
    .post(
        "/",
        async ({ body }) => {
            return writingService.add(body);
        },
        {
            isSignedIn: true,
        },
    )
    .patch(
        "/:id",
        async ({ status, body, params: { id } }) => {
            const res = await writingService.update(id, body);

            if (!res) {
                throw status(404, "Writing not found");
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
            const res = await writingService.delete(id);

            if (!res) {
                throw status(404, "Writing not found");
            }

            return res;
        },
        {
            isSignedIn: true,
        },
    );

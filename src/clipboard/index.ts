import Elysia from "elysia";
import { ClipboardService } from "./service";
import {bearer} from "@elysiajs/bearer";
import { auth } from "../auth";

const clipboardService = new ClipboardService();

export const clipboards = new Elysia({ prefix: "/clipboards"})
    .use(bearer())
    .use(auth)
    .get("/:id", async ({ status, params: {id}}) => {
        const res = await clipboardService.getClipboard(id);
        if (res.length === 0) throw status(404, "Clipboard not found");

        return res[0];
    })
    .get("/", () => clipboardService.getClipboards())
    .post(
        "/",
        async ({ body }) => {
            return clipboardService.add(body);
        },
        {
            isSignedIn: true,
        },
    )
    .patch(
        "/:id",
        async ({ status, body, params: { id } }) => {
            const res = await clipboardService.update(id, body);

            if (!res) {
                throw status(404, "Clipboard not found");
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
            const res = await clipboardService.delete(id);

            if (!res) {
                throw status(404, "Clipboard not found");
            }

            return "Deleted";
        },
        {
            isSignedIn: true,
        },
    );
;
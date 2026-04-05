import Elysia from "elysia";
import { WorkService } from "./service.js";

const workService = new WorkService();

export const works = new Elysia({ prefix: "/works" }).get("/", () =>
    workService.getWorks(),
);

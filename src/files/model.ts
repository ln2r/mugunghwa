import { t } from "elysia";

export interface Files {
    id?: string;
    key: string;
    created?: string;
    updated?: string;
    deleted?: string;
}

export const Files = t.Object({
    id: t.String(),
    key: t.String(),
    created: t.String(),
    updated: t.String(),
    deleted: t.String(),
});

export interface D1Return {
    success: boolean;
    meta: {
        served_by: string;
        duration: number;
        changes: number;
        last_row_id: number;
        change_db: boolean;
        size_after: number;
        rows_read: number;
        rows_written: number;
    };
    results: any[];
}

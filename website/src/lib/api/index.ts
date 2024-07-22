import type { ApiResponse, ResponseData } from "$lib/types";

export const UNEXPECTED_ERROR = { error: "Unexpected server response" };

export function resdata<D>({ res }: ApiResponse<D>): ResponseData<D> {
    if (res.type === "error") {
        return { error: res.error };
    }

    if (res.type !== "data") {
        return UNEXPECTED_ERROR;
    }

    return { data: res.data };
}

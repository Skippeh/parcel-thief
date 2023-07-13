import { ApiResponse, callApi } from ".";

export async function getSharedCargo(): Promise<ApiResponse<unknown>> {
  return await callApi<unknown>("baggages/sharedCargo", "GET", undefined);
}

import { ApiResponse, callApi } from ".";
import { ListSharedCargoResponse } from "../api_types";

export async function getSharedCargo(): Promise<
  ApiResponse<ListSharedCargoResponse>
> {
  return await callApi("baggages/sharedCargo", "GET", undefined);
}

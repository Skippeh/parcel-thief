import { ApiResponse, callApi } from ".";
import { ListLostCargoResponse, ListSharedCargoResponse } from "../api_types";

export async function getSharedCargo(): Promise<
  ApiResponse<ListSharedCargoResponse>
> {
  return await callApi("baggages/sharedCargo", "GET", undefined);
}

export async function getLostCargo(): Promise<
  ApiResponse<ListLostCargoResponse>
> {
  return await callApi("baggages/lostCargo", "GET", undefined);
}

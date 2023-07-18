import { ApiResponse, callApi } from ".";
import {
  ListLostCargoResponse,
  ListSharedCargoResponse,
  ListWastedCargoResponse,
} from "../api_types";

export async function getSharedCargo(): Promise<
  ApiResponse<ListSharedCargoResponse>
> {
  return await callApi("baggages/sharedCargo", "GET");
}

export async function getLostCargo(): Promise<
  ApiResponse<ListLostCargoResponse>
> {
  return await callApi("baggages/lostCargo", "GET");
}

export async function getWastedCargo(): Promise<
  ApiResponse<ListWastedCargoResponse>
> {
  return await callApi("baggages/wastedCargo", "GET");
}

import { ApiResponse, callApi } from ".";
import {
  Baggage,
  ListLostCargoResponse,
  ListSharedCargoResponse,
  ListWastedCargoResponse,
} from "../api_types";

export async function getSharedCargoList(): Promise<
  ApiResponse<ListSharedCargoResponse>
> {
  return await callApi("baggages/list/sharedCargo", "GET");
}

export async function getLostCargoList(): Promise<
  ApiResponse<ListLostCargoResponse>
> {
  return await callApi("baggages/list/lostCargo", "GET");
}

export async function getWastedCargoList(): Promise<
  ApiResponse<ListWastedCargoResponse>
> {
  return await callApi("baggages/list/wastedCargo", "GET");
}

export async function getBaggages(): Promise<ApiResponse<Baggage[]>> {
  return await callApi("baggages", "GET");
}

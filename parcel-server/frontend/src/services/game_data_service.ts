import { ApiResponse, callApi } from ".";
import { QpidArea } from "../api_types";

export function getQpidAreas(): Promise<ApiResponse<QpidArea[]>> {
  return callApi<QpidArea[]>("gameData/qpidAreas", "GET");
}

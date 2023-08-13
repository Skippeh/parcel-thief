import { ApiResponse, callApi } from ".";
import { Area, QpidObject } from "../api_types";

export function getQpidObjects(area: Area): Promise<ApiResponse<QpidObject[]>> {
  return callApi(`qpidObjects/${area}`, "GET");
}

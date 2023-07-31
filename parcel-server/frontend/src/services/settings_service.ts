import { ApiResponse, callApi } from ".";
import { SettingsValues } from "../api_types";

export async function getServerSettings(): Promise<
  ApiResponse<SettingsValues>
> {
  return await callApi("settings/server", "GET");
}

export async function setServerSettings(
  values: SettingsValues
): Promise<ApiResponse<SettingsValues>> {
  return await callApi("settings/server", "PUT", values);
}

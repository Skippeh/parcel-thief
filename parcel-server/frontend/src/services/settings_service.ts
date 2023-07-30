import { ApiResponse, callApi } from ".";
import { SettingsValues } from "../api_types";

export async function getSettings(): Promise<ApiResponse<SettingsValues>> {
  return await callApi("settings", "GET");
}

export async function setSettings(
  values: SettingsValues
): Promise<ApiResponse<SettingsValues>> {
  return await callApi("settings", "PUT", values);
}

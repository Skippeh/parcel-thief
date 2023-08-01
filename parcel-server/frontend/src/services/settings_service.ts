import { ApiResponse, callApi } from ".";
import { SettingsValues, WhitelistEntry } from "../api_types";

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

export async function getWhitelist(): Promise<ApiResponse<WhitelistEntry[]>> {
  return await callApi("settings/whitelist", "GET");
}

export async function setWhitelist(
  entries: WhitelistEntry[]
): Promise<ApiResponse<WhitelistEntry[]>> {
  return await callApi("settings/whitelist", "PUT", entries);
}

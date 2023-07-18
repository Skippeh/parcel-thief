import { ApiResponse, callApi } from ".";
import {
  FrontendPermissions,
  ListAccountsResponse,
  ListAccountsType,
} from "../api_types";

export async function getAccounts<
  T extends ListAccountsType,
  V extends { type: T } & ListAccountsResponse
>(type: T): Promise<ApiResponse<V>> {
  const query = `?accountsType=${encodeURIComponent(type)}`;
  return await callApi(`accounts${query}`, "GET", undefined);
}

export function permissionToReadableString(
  permission: FrontendPermissions | undefined | null
): string {
  console.log("permission", permission);

  switch (permission) {
    case "manageAccounts":
      return "Manage accounts";
    default:
      return permission ?? "Unknown";
  }
}
